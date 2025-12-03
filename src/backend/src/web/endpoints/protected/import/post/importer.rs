use ahash::HashSet;
use chrono::{Duration, NaiveTime, TimeDelta};
use color_eyre::{Report, Result, eyre::eyre};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use utoipa::ToSchema;

use crate::{
    types::{Availability, AvailabilityType, IsoDow, Lesson},
    web::endpoints::protected::import::post::{ImportFileMeta, ImportMode},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ScheduleFile {
    #[serde(rename = "LESSON")]
    lessons: Vec<RawLesson>,
}

/// A lesson in the schedule.
// <LESSON>
//   <DURATION>1:00</DURATION>
//   <SUBJECT>INFORMATICA</SUBJECT>
//   <SITE>Sede</SITE>
//   <MODULE>Standard</MODULE>
//   <TEACHER>SCIALPI MARIO</TEACHER>
//   <GROUP>5^A-IA</GROUP>
//   <ROOM>07-TW</ROOM>
//   <WEEK>A</WEEK>
//   <DAY>LUN</DAY>
//   <TIME>8:00</TIME>
// </LESSON>
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct RawLesson {
    duration: Option<String>,
    subject: Option<String>,
    #[serde(rename = "SITE")]
    _site: Option<String>,
    #[serde(rename = "MODULE")]
    _module: Option<String>,
    teacher: Option<Vec<String>>,
    group: Option<Vec<String>>,
    room: Option<Vec<String>>,
    #[serde(rename = "WEEK")]
    _week: Option<String>,
    #[serde(rename = "DAY")]
    ita_day: Option<ItaDay>,
    time: Option<NaiveTime>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
enum ItaDay {
    Lun = 1,
    Mar = 2,
    Mer = 3,
    Gio = 4,
    Ven = 5,
    Sab = 6,
    Dom = 7,
}

impl TryFrom<ItaDay> for IsoDow {
    type Error = Report;

    fn try_from(value: ItaDay) -> Result<Self> {
        match value {
            ItaDay::Lun => Ok(IsoDow::Mon),
            ItaDay::Mar => Ok(IsoDow::Tue),
            ItaDay::Mer => Ok(IsoDow::Wed),
            ItaDay::Gio => Ok(IsoDow::Thu),
            ItaDay::Ven => Ok(IsoDow::Fri),
            ItaDay::Sab => Ok(IsoDow::Sat),
            ItaDay::Dom => Ok(IsoDow::Sun),
        }
    }
}

// subject can be DISPO or RECUPERO_ORARIO
impl TryFrom<&str> for AvailabilityType {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "DISPO" => Ok(Self::Availability),
            "RECUPERO_ORARIO" => Ok(Self::RecoveryHours),
            _ => Err(color_eyre::eyre::eyre!(
                "Invalid availability type: {}",
                value
            )),
        }
    }
}

impl TryFrom<RawLesson> for Availability {
    type Error = Report;

    fn try_from(raw: RawLesson) -> Result<Self> {
        let availability_type = raw.subject.map(|s| s.as_str().try_into()).transpose()?;

        Ok(Self {
            teacher: raw.teacher,
            day: raw.ita_day.map(|d| d.try_into()).transpose()?,
            time: raw.time,
            availability_type,
        })
    }
}

impl TryFrom<RawLesson> for Lesson {
    type Error = Report;

    fn try_from(raw: RawLesson) -> Result<Self> {
        let duration: Option<TimeDelta> = raw
            .duration
            .as_ref()
            .map(|d| {
                let parts: Vec<&str> = d.split(':').collect();
                if parts.len() != 2 {
                    return Err(eyre!("Invalid duration format: {}", d));
                }
                let hours: i64 = parts[0]
                    .parse()
                    .map_err(|_| eyre!("Invalid hours in duration: {}", d))?;
                let minutes: i64 = parts[1]
                    .parse()
                    .map_err(|_| eyre!("Invalid minutes in duration: {}", d))?;
                Ok(Duration::hours(hours) + Duration::minutes(minutes))
            })
            .transpose()?;

        Ok(Self {
            duration,
            // take the first teacher if any
            teacher: raw.teacher.and_then(|t| t.into_iter().next()),
            day: raw.ita_day.map(|d| d.try_into()).transpose()?,
            time: raw.time,
            // take the first group if any
            group: raw.group.and_then(|g| g.into_iter().next()),
            // take the first room if any
            room: raw.room.and_then(|r| r.into_iter().next()),
        })
    }
}

pub async fn import_file(
    db: &PgPool,
    meta: ImportFileMeta,
    schedule_file: ScheduleFile,
    user_id: i32,
) -> Result<()> {
    let raw_lessons: Vec<RawLesson> = schedule_file.lessons;

    let mut txn = db.begin().await?;

    let import_id = create_import_record(&meta, user_id, &mut txn).await?;

    import_rooms(&raw_lessons, import_id, &mut txn).await?;

    import_groups(&raw_lessons, import_id, &mut txn).await?;

    import_teachers(&raw_lessons, import_id, &mut txn).await?;

    import_availabilities(raw_lessons.clone(), import_id, &mut txn).await?;

    import_lessons(raw_lessons, import_id, &mut txn).await?;

    match meta.mode {
        ImportMode::Write => txn.commit().await,
        ImportMode::DryRun => txn.rollback().await,
    }?;

    Ok(())
}

async fn create_import_record(
    import_file_meta: &ImportFileMeta,
    user_id: i32,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<i32> {
    let import_id = sqlx::query!(
        r#"
        INSERT INTO "import" (user_id, file_name, begin_ts, end_ts)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        user_id,
        import_file_meta.file_name,
        import_file_meta.begin_ts,
        import_file_meta.end_ts
    )
    .fetch_one(&mut **txn)
    .await?;

    Ok(import_id.id)
}

async fn import_rooms(
    raw_lessons: &[RawLesson],
    import_id: i32,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<()> {
    let all_rooms: Vec<&String> = raw_lessons
        .iter()
        .filter_map(|lesson| lesson.room.as_ref())
        .flatten()
        .collect::<HashSet<&String>>()
        .into_iter()
        .collect();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO "room" (name, import_id)
        "#,
    );

    query_builder.push_values(all_rooms, |mut b, room| {
        b.push_bind(room);
        b.push_bind(import_id);
    });

    query_builder.build().execute(&mut **txn).await?;

    Ok(())
}

async fn import_groups(
    raw_lessons: &[RawLesson],
    import_id: i32,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<()> {
    let all_groups: Vec<&String> = raw_lessons
        .iter()
        .filter_map(|lesson| lesson.group.as_ref())
        .flatten()
        .collect::<HashSet<&String>>()
        .into_iter()
        .collect();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO "group" (name, import_id)
        "#,
    );

    query_builder.push_values(all_groups, |mut b, group| {
        b.push_bind(group);
        b.push_bind(import_id);
    });

    query_builder.build().execute(&mut **txn).await?;

    Ok(())
}

async fn import_lessons(
    raw_lessons: Vec<RawLesson>,
    import_id: i32,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<()> {
    let lessons = raw_lessons
        .into_iter()
        .filter(|lesson| {
            if let Some(subject) = &lesson.subject {
                subject != "DISPO" && subject != "RECUPERO_ORARIO"
            } else {
                true
            }
        })
        .map(|lesson| lesson.try_into())
        .collect::<Result<Vec<Lesson>>>()?;

    for lesson in lessons {
        let day = lesson
            .day
            .as_ref()
            .ok_or_else(|| eyre!("Lesson doesn't have a day: {:?}", lesson))?;

        sqlx::query!(
            r#"
            INSERT INTO "lesson" (day, time, duration, room_id, group_id, teacher_id)
            SELECT
              $1::smallint::isodow,
              $2,
              $3,
              (SELECT id FROM room WHERE name = $4 AND import_id = $7),
              (SELECT id FROM "group" WHERE name = $5 AND import_id = $7),
              (SELECT id FROM teacher WHERE full_name = $6 AND import_id = $7)
            "#,
            day.iso_dow(),
            lesson.time as Option<NaiveTime>,
            lesson.duration as Option<TimeDelta>,
            lesson.room.as_deref(),
            lesson.group.as_deref(),
            lesson.teacher.as_deref(),
            import_id
        )
        .execute(&mut **txn)
        .await?;
    }

    Ok(())
}

async fn import_availabilities(
    raw_lessons: Vec<RawLesson>,
    import_id: i32,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<()> {
    // Filter for lessons that have any room that starts with DISPOSIZIONE#
    let lessons = raw_lessons
        .into_iter()
        .filter(|lesson| {
            if let Some(rooms) = &lesson.room {
                rooms.iter().any(|room| room.starts_with("DISPOSIZIONE#"))
            } else {
                false
            }
        })
        .map(|lesson| lesson.try_into())
        .collect::<Result<Vec<Availability>>>()?;

    // assert that every lesson has exactly one teacher
    for lesson in &lessons {
        assert_eq!(
            lesson.teacher.as_ref().map_or(0, |v| v.len()),
            1,
            "Lesson doesn't have exactly one teacher: {:?}",
            lesson
        );
    }

    for lesson in &lessons {
        let day = lesson
            .day
            .as_ref()
            .ok_or_else(|| eyre!("Lesson doesn't have a day: {:?}", lesson))?;
        let teacher = lesson
            .teacher
            .as_ref()
            .and_then(|t| t.first())
            .ok_or_else(|| eyre!("Lesson doesn't have a teacher: {:?}", lesson))?;
        let availability_type = lesson
            .availability_type
            .as_ref()
            .ok_or_else(|| eyre!("Lesson doesn't have an availability type: {:?}", lesson))?;

        sqlx::query!(
            r#"
            INSERT INTO "availability" (day, time, availability_type, teacher_id)
            SELECT $1::smallint::isodow,
                   $2,
                   $3,
                   (SELECT id FROM teacher WHERE full_name = $4 AND import_id = $5)
            "#,
            day.iso_dow(),
            lesson.time as Option<NaiveTime>,
            availability_type as &AvailabilityType,
            teacher,
            import_id
        )
        .execute(&mut **txn)
        .await?;
    }

    Ok(())
}

async fn import_teachers(
    raw_lessons: &[RawLesson],
    import_id: i32,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<()> {
    let all_teachers: Vec<&String> = raw_lessons
        .iter()
        .filter_map(|lesson| lesson.teacher.as_ref())
        .flatten()
        .collect::<HashSet<&String>>()
        .into_iter()
        .collect();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO "teacher" (full_name, import_id)
        "#,
    );

    query_builder.push_values(all_teachers, |mut b, teacher| {
        b.push_bind(teacher);
        b.push_bind(import_id);
    });

    query_builder.build().execute(&mut **txn).await?;

    Ok(())
}

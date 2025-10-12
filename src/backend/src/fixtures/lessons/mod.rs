use crate::types::{AvailabilityType, Day, Lesson};
use ahash::HashSet;
use chrono::NaiveTime;
use color_eyre::{eyre::eyre, Report, Result};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::info;

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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawLesson {
    _duration: Option<String>,
    subject: Option<String>,
    _site: Option<String>,
    _module: Option<String>,
    teacher: Option<Vec<String>>,
    _group: Option<Vec<String>>,
    room: Option<Vec<String>>,
    _week: Option<String>,
    #[serde(rename = "DAY")]
    ita_day: Option<ItaDay>,
    time: Option<NaiveTime>,
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

impl TryFrom<RawLesson> for Lesson {
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum ItaDay {
    Lun,
    Mar,
    Mer,
    Gio,
    Ven,
    Sab,
    Dom,
}

impl TryFrom<ItaDay> for Day {
    type Error = Report;

    fn try_from(value: ItaDay) -> Result<Self> {
        match value {
            ItaDay::Lun => Ok(Day::Mon),
            ItaDay::Mar => Ok(Day::Tue),
            ItaDay::Mer => Ok(Day::Wed),
            ItaDay::Gio => Ok(Day::Thu),
            ItaDay::Ven => Ok(Day::Fri),
            ItaDay::Sab => Ok(Day::Sat),
            ItaDay::Dom => Ok(Day::Sun),
        }
    }
}

#[derive(Debug, Deserialize)]
struct LessonsRoot {
    #[serde(rename = "LESSON")]
    lessons: Vec<RawLesson>,
}

const PATH: &str = "./src/fixtures/lessons/orario/Orario Provvisorio 5 ore v5.xml";

pub async fn seed(db: &PgPool, write: bool) -> Result<()> {
    info!("Seeding the lessons table...");

    let file_content = tokio::fs::read_to_string(PATH).await?;
    let raw_lessons: Vec<RawLesson> =
        quick_xml::de::from_str::<LessonsRoot>(&file_content)?.lessons;

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
        .collect::<Result<Vec<Lesson>>>()?;

    // assert that every lesson has exactly one teacher
    for lesson in &lessons {
        assert_eq!(
            lesson.teacher.as_ref().map_or(0, |v| v.len()),
            1,
            "Lesson doesn't have exactly one teacher: {:?}",
            lesson
        );
    }

    let unique_teachers: Vec<&String> = lessons
        .iter()
        .filter_map(|lesson| lesson.teacher.as_ref().and_then(|t| t.first()))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let mut txn = db.begin().await?;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO "teacher" (full_name)
        "#,
    );

    query_builder.push_values(unique_teachers, |mut b, teacher| {
        b.push_bind(teacher);
    });

    query_builder.build().execute(&mut *txn).await?;

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
            SELECT $1, $2, $3, id FROM teacher WHERE full_name = $4
            "#,
            day as &Day,
            lesson.time,
            availability_type as &AvailabilityType,
            teacher
        )
        .execute(&mut *txn)
        .await?;
    }

    if write {
        txn.commit().await?;
    } else {
        txn.rollback().await?;
    }

    info!(
        "Availability and teacher tables seeded ({})",
        if write { "Committed" } else { "Rolled Back" }
    );

    Ok(())
}

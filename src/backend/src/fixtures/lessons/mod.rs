use ahash::HashSet;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};
use strum::Display;
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
#[derive(Debug, Deserialize, restructed::Models)]
#[serde(rename_all = "UPPERCASE")]
#[view(
    Lesson,
    fields(teacher, day, time),
    attributes_with = "deriveless",
    derive(Serialize, Debug)
)]
struct RawLesson {
    _duration: Option<String>,
    _subject: Option<String>,
    _site: Option<String>,
    _module: Option<String>,
    teacher: Option<Vec<String>>,
    _group: Option<Vec<String>>,
    room: Option<Vec<String>>,
    _week: Option<String>,
    day: Option<Day>,
    time: Option<NaiveTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, sqlx::Type)]
#[sqlx(type_name = "day")]
#[serde(rename_all = "UPPERCASE")]
enum Day {
    Lun,
    Mar,
    Mer,
    Gio,
    Ven,
    Sab,
}

#[derive(Debug, Deserialize)]
struct LessonsRoot {
    #[serde(rename = "LESSON")]
    lessons: Vec<RawLesson>,
}

const PATH: &str = "./src/fixtures/lessons/orario/Orario Provvisorio 5 ore v5.xml";

pub async fn seed(db: &PgPool, write: bool) -> color_eyre::Result<()> {
    info!("Seeding the lessons table...");

    let file_content = tokio::fs::read_to_string(PATH).await?;
    let raw_lessons: Vec<RawLesson> =
        quick_xml::de::from_str::<LessonsRoot>(&file_content)?.lessons;

    // Filter for lessons that have any room that starts with DISPOSIZIONE#
    let lessons: Vec<Lesson> = raw_lessons
        .into_iter()
        .filter(|lesson| {
            if let Some(rooms) = &lesson.room {
                rooms.iter().any(|room| room.starts_with("DISPOSIZIONE#"))
            } else {
                false
            }
        })
        .map(|lesson| lesson.into())
        .collect();

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
        let day = lesson.day.as_ref().unwrap();
        let teacher = lesson.teacher.as_ref().and_then(|t| t.first()).unwrap();

        sqlx::query!(
            r#"
            INSERT INTO "availability" (day, time, teacher_id)
            SELECT $1, $2, id FROM teacher WHERE full_name = $3
            "#,
            day as &Day,
            lesson.time,
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

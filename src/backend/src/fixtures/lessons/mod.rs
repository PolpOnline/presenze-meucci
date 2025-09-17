use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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
    time: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

pub async fn seed(_db: &PgPool, _write: bool) -> color_eyre::Result<()> {
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

    println!("{}", sonic_rs::to_string_pretty(&lessons)?);

    Ok(())
}

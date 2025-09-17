use std::path::Path;

use serde::Deserialize;
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
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Lesson {
    duration: Option<String>,
    subject: Option<String>,
    site: Option<String>,
    module: Option<String>,
    teacher: Option<Vec<String>>,
    group: Option<Vec<String>>,
    room: Option<Vec<String>>,
    week: Option<String>,
    day: Option<String>,
    time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LessonsRoot {
    #[serde(rename = "LESSON")]
    lessons: Vec<Lesson>,
}

const PATH: &str = "./src/fixtures/lessons/orario/Orario Provvisorio 5 ore  v5.xml";

pub async fn seed(_db: &PgPool, _write: bool) -> color_eyre::Result<()> {
    info!("Seeding the lessons table...");

    let file_content = tokio::fs::read_to_string(PATH).await?;
    let lessons_root: LessonsRoot = quick_xml::de::from_str(&file_content)?;

    println!("{:?}", lessons_root.lessons);

    Ok(())
}

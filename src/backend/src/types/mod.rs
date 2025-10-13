use axum_serde::macros::{Deserialize, Serialize};
use chrono::NaiveTime;
use strum::Display;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Display, sqlx::Type)]
#[sqlx(type_name = "day")]
#[serde(rename_all = "UPPERCASE")]
pub enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug, Serialize)]
pub struct Availability {
    pub teacher: Option<Vec<String>>,
    pub day: Option<Day>,
    pub time: Option<NaiveTime>,
    pub availability_type: Option<AvailabilityType>,
}

#[derive(Debug, Serialize)]
pub struct Lesson {
    pub teacher: Option<String>,
    pub day: Option<Day>,
    pub time: Option<NaiveTime>,
    pub room: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, sqlx::Type)]
#[sqlx(type_name = "availability_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum AvailabilityType {
    Availability,
    RecoveryHours,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, sqlx::Type, ToSchema)]
#[sqlx(type_name = "absence_status")]
pub enum AbsenceStatus {
    Uncovered,
    ClassDelayed,
    ClassCancelled,
    SubstituteFound,
}

use axum_serde::macros::{Deserialize, Serialize};
use chrono::{NaiveTime, TimeDelta};
use strum::Display;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum IsoDow {
    Mon = 1,
    Tue = 2,
    Wed = 3,
    Thu = 4,
    Fri = 5,
    Sat = 6,
    Sun = 7,
}

impl IsoDow {
    pub fn iso_dow(&self) -> i16 {
        self.clone() as i16
    }
}

#[derive(Debug, Serialize)]
pub struct Availability {
    pub teacher: Option<Vec<String>>,
    pub day: Option<IsoDow>,
    pub time: Option<NaiveTime>,
    pub availability_type: Option<AvailabilityType>,
}

#[derive(Debug, Serialize)]
pub struct Lesson {
    pub teacher: Option<String>,
    pub day: Option<IsoDow>,
    pub time: Option<NaiveTime>,
    pub room: Option<String>,
    pub group: Option<String>,
    pub duration: Option<TimeDelta>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, sqlx::Type, ToSchema)]
#[sqlx(type_name = "availability_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum AvailabilityType {
    Availability,
    RecoveryHours,
}

#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, Display, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "absence_status")]
#[serde(rename_all = "camelCase")]
pub enum AbsenceStatus {
    #[default]
    Uncovered,
    ClassDelayed,
    ClassCanceled,
    SubstituteFound,
}

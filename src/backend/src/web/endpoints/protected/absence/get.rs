use ahash::AHashMap;
use axum::{extract::Query, response::IntoResponse};
use axum_serde::Sonic;
use chrono::{NaiveDate, NaiveTime};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::DASHBOARD_TAG, types::AbsenceStatus, users::AuthSession};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetAbsenceRequest {
    date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema)]
struct Absence {
    absent_professor: String,
    classes: Vec<AbsentClasses>,
}

#[derive(Debug, Serialize, ToSchema)]
struct AbsentClasses {
    id: i32,
    substitute_professor: Option<String>,
    time: NaiveTime,
    room: Option<String>,
    group: Option<String>,
    absent_status: AbsenceStatus,
}

#[utoipa::path(
    get,
    path = "/",
    summary = "Added absences",
    params(GetAbsenceRequest),
    responses(
        (status = OK, description = "Absences and their status", body = Vec<Absence>),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
    ),
    security(
        ("session" = [])
    ),
    tag = DASHBOARD_TAG,
)]
pub async fn get(
    Query(req): Query<GetAbsenceRequest>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    let rows = match sqlx::query!(
        r#"
        WITH active_import AS (SELECT id
                       FROM import
                       WHERE user_id = $2
                         AND begin_ts <= COALESCE($1, CURRENT_DATE)
                         AND end_ts >= COALESCE($1, CURRENT_DATE)
                       ORDER BY import_ts DESC
                       LIMIT 1)
        SELECT ab.id        AS id,
               t.full_name  AS absent_professor,
               t.id         AS absent_professor_id,
               l.time       AS time,
               r.name       AS room,
               g.name       AS "group",
               ab.status    AS "absent_status: AbsenceStatus",
               st.full_name AS substitute_professor
        FROM absence ab
                 JOIN lesson l ON ab.absent_teacher_lesson = l.id
                 JOIN teacher t ON l.teacher_id = t.id
                 JOIN active_import ON t.import_id = active_import.id
                 LEFT JOIN room r ON l.room_id = r.id
                 LEFT JOIN "group" g ON l.group_id = g.id
                 LEFT JOIN availability av ON ab.substitute_teacher_availability = av.id
                 LEFT JOIN teacher st ON av.teacher_id = st.id
        WHERE ab.absence_date = COALESCE($1, CURRENT_DATE);
        "#,
        req.date,
        user.id
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch absences with classes: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    // Group by (absence ID, absent professor) to form the final structure
    let absences: Vec<Absence> = rows
        .into_iter()
        .fold(AHashMap::new(), |mut acc, row| {
            let entry = acc
                .entry(row.absent_professor_id)
                .or_insert_with(|| Absence {
                    absent_professor: row.absent_professor,
                    classes: Vec::new(),
                });

            entry.classes.push(AbsentClasses {
                id: row.id,
                substitute_professor: row.substitute_professor,
                time: row.time,
                room: row.room,
                group: row.group,
                absent_status: row.absent_status,
            });

            acc
        })
        .into_values()
        .collect();

    Sonic(absences).into_response()
}

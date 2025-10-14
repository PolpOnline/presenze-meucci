use axum::{extract::Path, response::IntoResponse};
use axum_serde::Sonic;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::DASHBOARD_TAG, types::AvailabilityType, users::AuthSession};

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetCanBeAbsentRequest {
    absence_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableTeacher {
    id: i32,
    full_name: String,
    availability_type: AvailabilityType,
}

#[utoipa::path(
    get,
    path = "/available/{absence_id}",
    summary = "Available teachers for an absence",
    params(GetCanBeAbsentRequest),
    responses(
        (status = OK, description = "Available Teachers and their availability type", body = Vec<AvailableTeacher>),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
    ),
    security(
        ("session" = [])
    ),
    tag = DASHBOARD_TAG,
)]
pub async fn available(
    Path(req): Path<GetCanBeAbsentRequest>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    let available_teachers = match sqlx::query_as!(
        AvailableTeacher,
        r#"
        SELECT DISTINCT teacher.id,
                teacher.full_name,
                availability.availability_type as "availability_type: AvailabilityType"
        FROM absence
                 JOIN lesson absent_lesson ON absence.absent_teacher_lesson = absent_lesson.id
                 JOIN teacher absent_teacher ON absent_teacher.id = absent_lesson.teacher_id
                 JOIN import active_import ON active_import.id = absent_teacher.import_id
            AND absence.absence_date BETWEEN active_import.begin_ts AND active_import.end_ts
                 JOIN teacher ON teacher.import_id = active_import.id
                 JOIN availability ON availability.teacher_id = teacher.id
        WHERE absence.id = $2
          AND availability.day = absent_lesson.day
          AND availability.time = absent_lesson.time
          AND active_import.user_id = $1
        "#,
        user.id,
        req.absence_id,
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            error!("Database error when fetching available teachers: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    Sonic(available_teachers).into_response()
}

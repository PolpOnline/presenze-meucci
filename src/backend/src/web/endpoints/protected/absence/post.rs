use axum::response::IntoResponse;
use axum_serde::Sonic;
use chrono::{NaiveDate, NaiveTime};
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use utoipa::ToSchema;

use crate::{app::openapi::DASHBOARD_TAG, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddAbsenceRequest {
    /// The date of the absence. If not provided, defaults to today.
    date: Option<NaiveDate>,
    absent_teacher_id: i32,
    begin_time: NaiveTime,
    end_time: NaiveTime,
}

#[utoipa::path(
    post,
    path = "/",
    summary = "Add an absence",
    request_body = AddAbsenceRequest,
    responses(
        (status = OK, description = "Absence added"),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
        (status = BAD_REQUEST, description = "Invalid input")
    ),
    security(("session" = [])),
    tag = DASHBOARD_TAG,
)]
pub async fn post(
    auth_session: AuthSession,
    Sonic(req): Sonic<AddAbsenceRequest>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    if req.begin_time > req.end_time {
        return (StatusCode::BAD_REQUEST, "begin_ts must be before end_ts").into_response();
    }

    let res = sqlx::query!(
        r#"
        WITH lessons AS (
          SELECT le.id
          FROM lesson le
          JOIN teacher t ON le.teacher_id = t.id
          JOIN import i ON t.import_id = i.id
          WHERE le.teacher_id = $1
            AND le.day = EXTRACT(DOW FROM COALESCE($2, CURRENT_DATE)::date)::int
            AND le.time::time BETWEEN ($3::time) AND ($4::time)
            AND i.user_id = $5
        )
        INSERT INTO absence (absent_teacher_lesson, absence_date)
        SELECT l.id, ($2::date)
        FROM lessons l;
        "#,
        req.absent_teacher_id,
        req.date,
        req.begin_time,
        req.end_time,
        user.id
    )
    .execute(&auth_session.backend.db)
    .await;

    match res {
        Ok(done) if done.rows_affected() == 1 => StatusCode::OK.into_response(),
        Ok(_) => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
        Err(e) => {
            error!("Failed to add absence: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add absence").into_response()
        }
    }
}

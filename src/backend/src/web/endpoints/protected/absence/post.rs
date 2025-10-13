use crate::app::openapi::DASHBOARD_TAG;
use crate::users::AuthSession;
use axum::response::IntoResponse;
use axum_serde::Sonic;
use chrono::NaiveDateTime;
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddAbsenceRequest {
    absent_teacher_id: i32,
    begin_ts: NaiveDateTime,
    _end_ts: NaiveDateTime,
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

    if req.begin_ts > req._end_ts {
        return (StatusCode::BAD_REQUEST, "begin_ts must be before end_ts").into_response();
    }

    let res = sqlx::query!(
        r#"
        INSERT INTO absence (absent_teacher_lesson, absence_date)
        SELECT $1, COALESCE($2::timestamp, CURRENT_DATE)::date
        WHERE EXISTS (
            SELECT 1
            FROM lesson le
            JOIN teacher t ON le.teacher_id = t.id
            JOIN import i ON t.import_id = i.id
            WHERE le.id = $1 AND i.user_id = $3
        )
        "#,
        req.absent_teacher_id,
        // TODO: Use end_ts to create multiple absences
        req.begin_ts,
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

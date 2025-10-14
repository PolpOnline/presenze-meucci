use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use utoipa::ToSchema;

use crate::{app::openapi::DASHBOARD_TAG, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAbsenceRequest {
    id: i32,
}

#[utoipa::path(
    delete,
    path = "/",
    summary = "Delete absence",
    request_body = DeleteAbsenceRequest,
    responses(
        (status = OK, description = "Deleted absence"),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
        (status = NOT_FOUND, description = "Absence not found or not accessible"),
    ),
    security(("session" = [])),
    tag = DASHBOARD_TAG,
)]
pub async fn delete(
    auth_session: AuthSession,
    Sonic(req): Sonic<DeleteAbsenceRequest>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    match sqlx::query!(
        r#"
        DELETE FROM absence ab
        USING lesson l, teacher t, import i
        WHERE ab.id = $1
          AND ab.absent_teacher_lesson = l.id
          AND l.teacher_id = t.id
          AND t.import_id = i.id
          AND i.user_id = $2
        "#,
        req.id,
        user.id
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(done) if done.rows_affected() >= 1 => StatusCode::OK.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Absence not found").into_response(),
        Err(e) => {
            error!("Failed to delete absence: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

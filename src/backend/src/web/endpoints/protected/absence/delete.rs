use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;
use serde::Deserialize;
use tracing::error;
use utoipa::IntoParams;

use crate::{app::openapi::DASHBOARD_TAG, users::AuthSession};

#[derive(Debug, Deserialize, IntoParams)]
pub struct DeleteAbsencePathParams {
    absence_id: i32,
}

#[utoipa::path(
    delete,
    path = "/{absence_id}",
    summary = "Delete absence",
    params(DeleteAbsencePathParams),
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
    Path(req): Path<DeleteAbsencePathParams>,
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
        req.absence_id,
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

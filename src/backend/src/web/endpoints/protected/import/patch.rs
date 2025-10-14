use axum::response::IntoResponse;
use axum_serde::{macros::Deserialize, Sonic};
use chrono::NaiveDateTime;
use http::StatusCode;
use tracing::error;
use utoipa::ToSchema;

use crate::{app::openapi::IMPORT_TAG, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImportPatchRequest {
    id: i32,
    begin_ts: Option<NaiveDateTime>,
    end_ts: Option<NaiveDateTime>,
}

#[utoipa::path(
    patch,
    path = "/",
    summary = "Modify an import's metadata",
    request_body = ImportPatchRequest,
    responses(
        (status = OK, description = "The import was patched"),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
    ),
    security(
        ("session" = [])
    ),
    tag = IMPORT_TAG,
)]
pub async fn patch(
    auth_session: AuthSession,
    Sonic(req): Sonic<ImportPatchRequest>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    match sqlx::query!(
        r#"
        UPDATE import
        SET begin_ts = COALESCE($2, begin_ts),
            end_ts = COALESCE($3, end_ts)
        WHERE id = $1 AND user_id = $4
        "#,
        req.id,
        req.begin_ts,
        req.end_ts,
        user.id,
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Database error when patching the import: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    StatusCode::OK.into_response()
}

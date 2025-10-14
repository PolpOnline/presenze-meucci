use axum::{extract::Path, response::IntoResponse};
use axum_serde::{Sonic, macros::Deserialize};
use chrono::NaiveDateTime;
use http::StatusCode;
use tracing::error;
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::IMPORT_TAG, users::AuthSession};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ImportPatchPathParams {
    import_id: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImportPatchRequest {
    begin_ts: Option<NaiveDateTime>,
    end_ts: Option<NaiveDateTime>,
}

#[utoipa::path(
    patch,
    path = "/{import_id}",
    summary = "Modify an import's metadata",
    params(ImportPatchPathParams),
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
    Path(path): Path<ImportPatchPathParams>,
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
        path.import_id,
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

use axum::{extract::Path, response::IntoResponse};
use axum_serde::macros::Deserialize;
use http::StatusCode;
use tracing::error;
use utoipa::IntoParams;

use crate::{app::openapi::IMPORT_TAG, users::AuthSession};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ImportDeletionPathParams {
    import_id: i32,
}

#[utoipa::path(
    delete,
    path = "/{import_id}",
    summary = "Delete an import",
    params(ImportDeletionPathParams),
    responses(
        (status = OK, description = "The import was deleted"),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
    ),
    security(
        ("session" = [])
    ),
    tag = IMPORT_TAG,
)]
pub async fn delete(
    auth_session: AuthSession,
    Path(req): Path<ImportDeletionPathParams>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    match sqlx::query!(
        r#"
        DELETE FROM import
        WHERE id = $1 AND user_id = $2
        "#,
        req.import_id,
        user.id,
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Database error when deleting the import: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    StatusCode::OK.into_response()
}

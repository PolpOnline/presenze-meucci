use axum::response::IntoResponse;
use axum_serde::Sonic;
use chrono::NaiveDateTime;
use http::StatusCode;
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use crate::{app::openapi::IMPORT_TAG, users::AuthSession};

#[derive(Debug, Serialize, ToSchema)]
struct ImportInfo {
    id: i32,
    file_name: String,
    begin_ts: NaiveDateTime,
    end_ts: NaiveDateTime,
}

#[utoipa::path(
    get,
    path = "/",
    summary = "List imports",
    responses(
        (status = OK, description = "List of imports", body = Vec<ImportInfo>),
        (status = UNAUTHORIZED, description = "Unauthorized", example = "Unauthorized"),
    ),
    security(
        ("session" = [])
    ),
    tag = IMPORT_TAG,
)]
pub async fn get(auth_session: AuthSession) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    let imports = match sqlx::query_as!(
        ImportInfo,
        r#"
        SELECT i.id, i.file_name, i.begin_ts, i.end_ts
        FROM import i
        WHERE i.user_id = $1
        ORDER BY i.begin_ts DESC
        "#,
        user.id,
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            error!("Database error when getting the imports: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    Sonic(imports).into_response()
}

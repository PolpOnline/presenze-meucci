use axum::response::IntoResponse;
use http::StatusCode;

use crate::{app::openapi::DEFAULT_TAG, users::AuthSession};

#[utoipa::path(
    get,
    path = "/ok",
    summary = "ok",
    responses(
        (status = OK, description = "OK", example = "OK"),
    ),
    security(
        ("session" = [])
    ),
    tag = DEFAULT_TAG,
)]
pub async fn ok(_auth_session: AuthSession) -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}

use axum::response::IntoResponse;
use axum_serde::Sonic;
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use thiserror::Error;
use tracing::{info};
use utoipa::ToSchema;

use crate::{
    app::openapi::AUTH_TAG,
    users::{AuthSession, Credentials},
};

#[derive(Error, Debug, ErrorStatus, ToSchema)]
pub enum AuthError {
    #[error("Failed to generate hash")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToGenerateHash,
    #[error("Failed to insert user")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    #[schema(value_type = String)]
    FailedToInsertNewUser(#[from] sqlx::Error),
    #[error("User doesn't exist after signup")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    UserNotExistingAfterSignUp,
    #[error("Failed to re-authenticate after signup")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToReAuthenticateAfterSignUp,
    #[error("Wrong password")]
    #[status(StatusCode::UNAUTHORIZED)]
    WrongPassword,
}

#[utoipa::path(
    post,
    path = "/login",
    summary = "Login",
    description = "Login",
    responses(
        (status = OK, description = "User was logged in"),
        (status = NOT_FOUND, description = "User not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = str, content_type = "text/plain"),
        (status = UNAUTHORIZED, description = "Wrong password", body = str, content_type = "text/plain")
    ),
    tag = AUTH_TAG
)]
pub async fn login(
    mut auth_session: AuthSession,
    Sonic(req): Sonic<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(req.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return AuthError::WrongPassword.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    info!("Successfully logged in as {}", user.username);
    StatusCode::OK.into_response()
}


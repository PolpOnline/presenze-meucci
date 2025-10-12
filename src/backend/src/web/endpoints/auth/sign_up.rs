use crate::users::User;
use crate::web::endpoints::auth::login::AuthError;
use crate::{
    app::openapi::AUTH_TAG,
    users::{AuthSession, Credentials},
};
use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use password_auth::generate_hash;
use tokio::task;
use tracing::debug;

#[utoipa::path(
    post,
    path = "/sign_up",
    summary = "Sign Up",
    description = "Sign up and login automatically",
    responses(
        (status = OK, description = "User signed up and logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = str, content_type = "text/plain"),
    ),
    tag = AUTH_TAG
)]
pub async fn sign_up(
    mut auth_session: AuthSession,
    Sonic(req): Sonic<Credentials>,
) -> impl IntoResponse {
    let password = req.password.clone();

    let Ok(encrypted_password) =
        task::spawn_blocking(move || generate_hash(password.as_bytes())).await
    else {
        return AuthError::FailedToGenerateHash.into_response();
    };

    let user = match sqlx::query_as!(
        User,
        r#"
        INSERT INTO "user" (username, password)
        VALUES ($1, $2)
        RETURNING id, username, password;
        "#,
        req.username,
        encrypted_password,
    )
    .fetch_one(&auth_session.backend.db)
    .await
    {
        Ok(user) => user,
        Err(e) => return AuthError::FailedToInsertNewUser(e).into_response(),
    };

    match auth_session.authenticate(req.clone()).await {
        Ok(Some(_)) => {},
        Ok(None) => {
            debug!("User does not exist after signup");
            return AuthError::UserNotExistingAfterSignUp.into_response()
        }
        Err(_) => return AuthError::FailedToReAuthenticateAfterSignUp.into_response(),
    }

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    StatusCode::OK.into_response()
}
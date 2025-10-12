mod login;
mod logout;
mod sign_up;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes![login::login])
        .routes(routes![sign_up::sign_up])
        .routes(routes![logout::logout])
}

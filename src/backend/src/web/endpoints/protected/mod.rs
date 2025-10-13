pub mod import;
mod ok;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(ok::ok))
        .nest("/import", import::router())
}

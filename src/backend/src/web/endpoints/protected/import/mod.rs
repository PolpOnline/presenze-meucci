use utoipa_axum::{router::OpenApiRouter, routes};

pub mod post;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(post::post))
}

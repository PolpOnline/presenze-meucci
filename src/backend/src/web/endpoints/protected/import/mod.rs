use utoipa_axum::{router::OpenApiRouter, routes};

mod get;
pub mod post;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(post::post, get::get))
}

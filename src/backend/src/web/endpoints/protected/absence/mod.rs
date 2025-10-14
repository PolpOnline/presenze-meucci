use utoipa_axum::{router::OpenApiRouter, routes};

mod delete;
pub mod get;
mod post;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(get::get, post::post, delete::delete))
}

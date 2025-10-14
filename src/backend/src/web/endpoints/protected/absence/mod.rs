use utoipa_axum::{router::OpenApiRouter, routes};

pub mod get;
mod post;
mod delete;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(get::get, post::post, delete::delete))
}

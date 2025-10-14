use utoipa_axum::{router::OpenApiRouter, routes};

mod delete;
mod get;
pub mod post;
mod patch;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(post::post, get::get, delete::delete, patch::patch))
}

use utoipa_axum::{router::OpenApiRouter, routes};

mod delete;
mod get;
mod patch;
pub mod post;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(post::post, get::get, delete::delete, patch::patch))
}

use utoipa_axum::{router::OpenApiRouter, routes};

mod file;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(file::file))
}

use utoipa_axum::{router::OpenApiRouter, routes};

mod available;
mod can_be_absent;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(can_be_absent::can_be_absent))
        .routes(routes!(available::available))
}

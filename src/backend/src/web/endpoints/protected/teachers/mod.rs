use utoipa_axum::{router::OpenApiRouter, routes};

mod can_be_absent;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(can_be_absent::can_be_absent))
}

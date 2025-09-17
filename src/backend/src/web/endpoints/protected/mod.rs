mod ok;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(ok::ok))
}

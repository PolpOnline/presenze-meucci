pub mod import;
mod absence;

use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/absence", absence::router())
        .nest("/import", import::router())
}

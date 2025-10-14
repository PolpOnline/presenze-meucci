mod absence;
pub mod import;
mod teachers;

use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/absence", absence::router())
        .nest("/import", import::router())
        .nest("/teachers", teachers::router())
}

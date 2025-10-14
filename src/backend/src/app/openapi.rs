use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

use crate::web::endpoints::protected::import::post::ImportMode;

pub const DEFAULT_TAG: &str = "Default";
pub const AUTH_TAG: &str = "Authentication";
pub const IMPORT_TAG: &str = "Import";
pub const DASHBOARD_TAG: &str = "Dashboard";

// ImportMode specification is a fix for https://github.com/juhaku/utoipa/issues/1165
#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocSecurityAddon),
    tags(
        (name = DEFAULT_TAG, description = "Default tag"),
        (name = AUTH_TAG, description = "Authentication related endpoints"),
        (name = IMPORT_TAG, description = "Import related endpoints"),
        (name = DASHBOARD_TAG, description = "Dashboard related endpoints"),
    ),
    components(
        schemas(
            ImportMode
        )
    ),
)]
pub(super) struct ApiDoc;

struct ApiDocSecurityAddon;

impl Modify for ApiDocSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("meucci_presenze_id"))),
            )
        }
    }
}

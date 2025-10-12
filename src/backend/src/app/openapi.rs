use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

pub const DEFAULT_TAG: &str = "Default";
pub const AUTH_TAG: &str = "Authentication";

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocSecurityAddon),
    tags(
        (name = DEFAULT_TAG, description = "Default tag"),
        (name = AUTH_TAG, description = "Authentication related endpoints")
    )
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

use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi(modifiers(&SecurityAddon), paths(), tags((name="Auth", description="Authentication")))]
struct ApiDoc;

struct SecurityAddon;
// use crate::api::v1::auth::router
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(security) = openapi.components.as_mut() {
            security.add_security_scheme(
                "BearerAuth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(
                    "api_key",
                ))),
            );
        }
    }
}
use crate::routes;
pub async fn app() -> Router {
    let api = ApiDoc::openapi();
    let (router, api) = routes::add_routes(api.clone());
    let router = router.merge(Redoc::with_url("/redoc", api.clone()));
    return router;
}

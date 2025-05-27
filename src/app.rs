use axum::{Json, Router, response::IntoResponse};
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
    let router = router
        .route("/health", axum::routing::get(health_check))
        .merge(Redoc::with_url("/redoc", api.clone()));
    return router;
}

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Spendlite API is running";

    // check ping to ensure the service is up
    // In a real application, you might want to check database connections, external services, etc.
    let start = std::time::Instant::now();
    let ping =
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            // Simulate a ping operation
            // In a real application, you might want to ping a database or an external service
            Ok::<_, ()>(())
        })
        .await;

    // ping response time in milliseconds
    let ping_response_time = start.elapsed().as_millis();

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE,
        "ping": ping.is_ok(),
        "ping_response_time_ms": ping_response_time,
        "version": env!("CARGO_PKG_VERSION"),
    });

    Json(json_response)
}

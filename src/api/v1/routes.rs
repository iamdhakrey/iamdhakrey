use axum::Router;
use utoipa_axum::router::OpenApiRouter;

use crate::api::v1::auth;

/// Add routes to the OpenAPI documentation
/// and return the router and the OpenAPI documentation
/// Add Prefix '/api/v1' to the OpenAPI documentation
pub async fn v1_route(
    api: utoipa::openapi::OpenApi,
) -> (Router, utoipa::openapi::OpenApi) {
    let (router, api) = OpenApiRouter::with_openapi(api.clone())
        .nest("/api/v1", auth::router::auth_router().await)
        .split_for_parts();
    return (router, api);
}

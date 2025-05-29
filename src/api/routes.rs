use axum::Router;
use utoipa_axum::router::OpenApiRouter;

use super::v1::routes::v1_route;

/// Add routes to the OpenAPI documentation
/// and return the router and the OpenAPI documentation
pub async fn add_routes(
    api: utoipa::openapi::OpenApi,
) -> (Router, utoipa::openapi::OpenApi) {
    let (_, apis) = OpenApiRouter::<Router>::with_openapi(api.clone())
        .split_for_parts();
    let (v1_router, api) = v1_route(apis.clone()).await;
    return (v1_router, api);
}

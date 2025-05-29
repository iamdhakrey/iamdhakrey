use axum::Router;
use utoipa_axum::router::OpenApiRouter;

use crate::api::routes;
use crate::api::v1::auth::handlers::sign_in;

pub async fn add_routes(
    // router: Router,
    api: utoipa::openapi::OpenApi,
) -> (Router, utoipa::openapi::OpenApi) {
    let (router, api) = OpenApiRouter::with_openapi(api)
        // .route("/api/v1/auth/signin", sign_in)
        .split_for_parts();
    let (routes, api) = routes::add_routes(api.clone()).await;

    let router = router.merge(routes);
    // Create a new Axum application
    // let router = router.merge(Redoc::with_url("/redoc", api.clone()));
    // .merge(Redoc::with_url("/redoc", api.clone()))
    // let router = router.merge(auth::router::auth_router());
    return (router, api);
}

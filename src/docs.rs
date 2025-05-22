use axum::Router;
use utoipa_redoc::{Redoc, Servable};

pub fn route(router: Router, api: utoipa::openapi::OpenApi) -> Router {
    // Create a new Axum application
    // let router = router.merge(Redoc::with_url("/redoc", api.clone()));
    // .merge(Redoc::with_url("/redoc", api.clone()))
    // let router = router.merge(auth::router::auth_router());
    let router =
        router.merge(Redoc::with_url("/openapi.json", api.clone()));
    let router = router.merge(Redoc::with_url("/redoc", api.clone()));

    return router;
}

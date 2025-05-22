use axum::Router;
use utoipa_axum::router::OpenApiRouter;

// use crate::api::v1::auth;

use super::v1::routes::v1_route;

fn prefix_openapi_paths(
    mut api: utoipa::openapi::OpenApi,
    prefix: &str,
) -> utoipa::openapi::OpenApi {
    let mut new_paths = utoipa::openapi::path::Paths::new();
    println!("prefix_openapi_paths: {}", prefix);
    for (path, item) in api.paths.paths.into_iter() {
        let new_path = format!("{}{}", prefix, path);
        println!("{} -> {}", path, new_path);
        new_paths.paths.insert(new_path, item);
    }

    api.paths.paths = new_paths.paths;
    api
}

pub fn add_routes(
    api: utoipa::openapi::OpenApi,
) -> (Router, utoipa::openapi::OpenApi) {
    let (router, apis) =
        OpenApiRouter::with_openapi(api.clone()).split_for_parts();
    let (v1_router, api) = v1_route(apis.clone());
    let router = router.nest("/api", v1_router);

    let api = prefix_openapi_paths(api, "/api");
    return (router, api);
}

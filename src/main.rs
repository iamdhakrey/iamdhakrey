use axum::{self, Extension, http::Request};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnResponse, MakeSpan, TraceLayer,
};
use tracing::{Level, Span, info};

pub mod api;
mod app;
pub mod config;
mod db;
pub mod docs;
mod entities;
mod logger;
pub mod middleware;
pub mod models;
pub mod response;
mod routes;
mod state;

#[derive(Clone)]
pub struct ApiMakeSpan;

impl<B> MakeSpan<B> for ApiMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        tracing::span!(
            target: "api", // 👈 Set custom target here
            Level::INFO,
            "request",
            method = ?request.method(),
            uri = ?request.uri(),
            version = ?request.version()
        )
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = &config::CONFIG.read().unwrap();
    use std::env;
    unsafe {
        env::set_var(
            "RUST_LOG",
            "info,spendlite-api=info,tower_http=off,sqlx=off",
        );
    }
    let _log_gaurds = logger::init_logging(
        config.log_level.as_deref().unwrap_or("info"),
    );

    let state = init_db().await;

    info!("Starting Spendlite API...");
    let bind_address = format!(
        "{}:{}",
        config.host.clone().unwrap_or_else(|| "127.0.0.1".to_string()),
        config.port.clone().unwrap_or(3000)
    );
    let listener = TcpListener::bind(bind_address)
        .await
        .expect("Failed to bind to address");
    let listen =
        listener.local_addr().expect("Failed to get local address");
    info!("Listening on {}", listen);

    let router = app::app().await.layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            // .make_span_with(ApiMakeSpan) // .on_request(DefaultOnRequest) // .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    // Add the application state to the router
    let app = router
        .route(
            "/sign_in",
            axum::routing::post(api::v1::auth::handlers::sign_in),
        )
        .layer(Extension(state));

    // Start the server
    axum::serve(listener, app).await.expect("Failed to run server");
}

use std::sync::Arc;

pub async fn init_db() -> AppState {
    let config = &config::CONFIG.read().unwrap();
    let db_url = config.database_url.clone();
    let db = db::connect(&db_url).await;
    let state = AppState { db: Arc::new(db) };
    return state;
}

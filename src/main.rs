use axum;
use tokio::net::TcpListener;
pub mod api;
mod app;
pub mod middleware;
pub mod docs;
pub mod response;
mod routes;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    println!("Listening on 127.0.0.1:3000");

    let app = app::app().await;
    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");
    println!("Server started on 127.0.0.1:3000");
    // Wait for the server to finish
}

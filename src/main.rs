use axum::{
    Router,
};
use tower_http::services::ServeDir;
use tracing_subscriber::fmt::init;

mod api;

#[tokio::main]
async fn main() {
    init();

    let app = Router::new()
        .nest("/api", api::router())
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Rust Lab running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}


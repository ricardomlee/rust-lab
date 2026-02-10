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
        // serve wasm files at /wasm explicitly to ensure the worker can reach them
        .nest_service("/wasm", ServeDir::new("static/wasm"))
        .fallback_service(ServeDir::new("static"));

    // startup diagnostic: print whether the expected wasm JS is present
    let wasm_js_path = std::path::Path::new("static/wasm/wasm_lab.js");
    if wasm_js_path.exists() {
        println!("✅ Found {}", wasm_js_path.display());
    } else {
        println!("⚠️ Missing {} — worker fetch may 404", wasm_js_path.display());
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Rust Lab running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}


use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

mod text;
mod error;
mod base;

#[derive(Serialize)]
struct Health {
    status: &'static str,
    timestamp: u64,
}

async fn health() -> Json<Health> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Json(Health {
        status: "ok",
        timestamp: now,
    })
}

pub fn router() -> Router {
    Router::new()
    .route("/health", get(health))
    .route("/text/analyze", post(text::analyze))
    .route("/base/convert", post(base::convert))
}


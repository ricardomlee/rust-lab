use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

mod base;
mod bytes;
mod countdown;
mod duration;
mod error;
mod frame_time;
mod percent;
mod text;

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
        .route("/bytes/format", post(bytes::format))
        .route("/countdown/format", post(countdown::format))
        .route("/duration/format", post(duration::format))
        .route("/frame-time/format", post(frame_time::format))
        .route("/percent/format", post(percent::format))
}

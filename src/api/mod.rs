use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

mod base;
mod base64;
mod bytes;
mod color;
mod countdown;
mod duration;
mod error;
mod frame_time;
mod hash;
mod percent;
mod text;
mod uuid;

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
        .route("/color/hex-to-rgba", post(color::hex_to_rgba))
        .route("/color/rgb-to-hex", post(color::rgb_to_hex))
        .route("/color/rgb-to-hsl", post(color::rgb_to_hsl))
        .route("/color/hsl-to-rgb", post(color::hsl_to_rgb))
        .route("/base64/encode", post(base64::encode))
        .route("/base64/decode", post(base64::decode))
        .route("/hash/compute", post(hash::compute))
        .route("/uuid/generate", get(uuid::generate))
        .route("/uuid/short", get(uuid::short_id))
}

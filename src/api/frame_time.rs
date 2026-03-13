use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::error::ApiError;

#[derive(Deserialize)]
pub struct FrameTimeRequest {
    pub fps: f64,
}

#[derive(Serialize)]
pub struct FrameTimeResult {
    pub fps: f64,
    pub frame_ms: f64,
    pub label: String,
    pub quality_hint: String,
    pub elapsed_us: u128,
}

pub async fn format(Json(req): Json<FrameTimeRequest>) -> Result<Json<FrameTimeResult>, ApiError> {
    let start = Instant::now();
    let formatted = core::frame_time::format_frame_time(req.fps).map_err(ApiError::BadRequest)?;
    let elapsed_us = start.elapsed().as_micros();

    Ok(Json(FrameTimeResult {
        fps: formatted.fps,
        frame_ms: formatted.frame_ms,
        label: formatted.label,
        quality_hint: formatted.quality_hint,
        elapsed_us,
    }))
}

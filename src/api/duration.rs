use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::error::ApiError;

#[derive(Deserialize)]
pub struct DurationRequest {
    pub milliseconds: u64,
}

#[derive(Serialize)]
pub struct DurationResult {
    pub milliseconds: u64,
    pub compact: String,
    pub clock: String,
    pub verbose_zh: String,
    pub elapsed_us: u128,
}

pub async fn format(Json(req): Json<DurationRequest>) -> Result<Json<DurationResult>, ApiError> {
    let start = Instant::now();
    let formatted = core::duration::format_duration(req.milliseconds);
    let elapsed_us = start.elapsed().as_micros();

    Ok(Json(DurationResult {
        milliseconds: formatted.milliseconds,
        compact: formatted.compact,
        clock: formatted.clock,
        verbose_zh: formatted.verbose_zh,
        elapsed_us,
    }))
}

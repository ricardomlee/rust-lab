use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::error::ApiError;

#[derive(Deserialize)]
pub struct PercentRequest {
    pub value: f64,
}

#[derive(Serialize)]
pub struct PercentResult {
    pub value: f64,
    pub percent: String,
    pub ratio: String,
    pub hint: String,
    pub elapsed_us: u128,
}

pub async fn format(Json(req): Json<PercentRequest>) -> Result<Json<PercentResult>, ApiError> {
    let start = Instant::now();
    let formatted = core::percent::format_percent(req.value).map_err(ApiError::BadRequest)?;
    let elapsed_us = start.elapsed().as_micros();

    Ok(Json(PercentResult {
        value: formatted.value,
        percent: formatted.percent,
        ratio: formatted.ratio,
        hint: formatted.hint,
        elapsed_us,
    }))
}

use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::error::ApiError;

#[derive(Deserialize)]
pub struct BytesRequest {
    pub bytes: u64,
}

#[derive(Serialize)]
pub struct BytesResult {
    pub bytes: u64,
    pub binary: String,
    pub decimal: String,
    pub binary_per_second: String,
    pub decimal_per_second: String,
    pub elapsed_us: u128,
}

pub async fn format(Json(req): Json<BytesRequest>) -> Result<Json<BytesResult>, ApiError> {
    let start = Instant::now();
    let formatted = core::bytes::format_byte_size(req.bytes);
    let elapsed_us = start.elapsed().as_micros();

    Ok(Json(BytesResult {
        bytes: formatted.bytes,
        binary: formatted.binary,
        decimal: formatted.decimal,
        binary_per_second: formatted.binary_per_second,
        decimal_per_second: formatted.decimal_per_second,
        elapsed_us,
    }))
}

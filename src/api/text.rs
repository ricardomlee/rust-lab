use axum::Json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Instant;

use super::error::ApiError;

#[derive(Deserialize)]
pub struct TextRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct TextResult {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
    pub sha256: String,
    pub elapsed_us: u128,
}

pub async fn analyze(
    Json(req): Json<TextRequest>,
) -> Result<Json<TextResult>, ApiError> {
    if req.text.trim().is_empty() {
        return Err(ApiError::BadRequest("文本不能为空".into()));
    }

    let start = Instant::now();

    let chars = req.text.chars().count();
    let words = req.text.split_whitespace().count();
    let lines = req.text.lines().count();

    let mut hasher = Sha256::new();
    hasher.update(req.text.as_bytes());
    let sha256 = format!("{:x}", hasher.finalize());

    let elapsed_us = start.elapsed().as_micros();

    Ok(Json(TextResult {
        chars,
        words,
        lines,
        sha256,
        elapsed_us,
    }))
}

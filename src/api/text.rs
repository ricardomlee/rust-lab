use axum::Json;
use serde::{Deserialize, Serialize};
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

pub async fn analyze(Json(req): Json<TextRequest>) -> Result<Json<TextResult>, ApiError> {
    if req.text.trim().is_empty() {
        return Err(ApiError::BadRequest("文本不能为空".into()));
    }

    let start = Instant::now();

    let analysis = core::text::analyze(&req.text).map_err(ApiError::BadRequest)?;

    let elapsed_us = start.elapsed().as_micros();

    Ok(Json(TextResult {
        chars: analysis.chars,
        words: analysis.words,
        lines: analysis.lines,
        sha256: analysis.sha256,
        elapsed_us,
    }))
}

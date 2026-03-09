use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
pub struct CountdownRequest {
    pub total_seconds: i64,
}

#[derive(Serialize)]
pub struct CountdownResult {
    pub total_seconds: i64,
    pub sign: String,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub clock: String,
    pub compact: String,
    pub status: String,
    pub elapsed_us: u128,
}

pub async fn format(Json(req): Json<CountdownRequest>) -> Json<CountdownResult> {
    let start = Instant::now();
    let formatted = core::countdown::format_countdown(req.total_seconds);
    let elapsed_us = start.elapsed().as_micros();

    Json(CountdownResult {
        total_seconds: formatted.total_seconds,
        sign: formatted.sign,
        days: formatted.days,
        hours: formatted.hours,
        minutes: formatted.minutes,
        seconds: formatted.seconds,
        clock: formatted.clock,
        compact: formatted.compact,
        status: formatted.status,
        elapsed_us,
    })
}

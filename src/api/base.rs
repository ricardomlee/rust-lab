use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
pub struct BaseRequest {
    pub value: String,
    pub from: u32,
    pub to: u32,
}

#[derive(Serialize)]
pub struct BaseResponse {
    pub result: String,
    pub elapsed_us: u128,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn convert(
    Json(req): Json<BaseRequest>,
) -> Result<Json<BaseResponse>, Json<ErrorResponse>> {
    let start = Instant::now();

    match core::base::convert(&req.value, req.from, req.to) {
        Ok(result) => Ok(Json(BaseResponse {
            result,
            elapsed_us: start.elapsed().as_micros(),
        })),
        Err(e) => Err(Json(ErrorResponse { error: e })),
    }
}


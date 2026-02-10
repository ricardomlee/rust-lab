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

    if !(2..=36).contains(&req.from) || !(2..=36).contains(&req.to) {
        return Err(Json(ErrorResponse {
            error: "base must be between 2 and 36".into(),
        }));
    }

    let parsed = match i128::from_str_radix(&req.value, req.from) {
        Ok(v) => v,
        Err(e) => {
            return Err(Json(ErrorResponse {
                error: e.to_string(),
            }))
        }
    };

    let result = to_base(parsed, req.to);

    Ok(Json(BaseResponse {
        result,
        elapsed_us: start.elapsed().as_micros(),
    }))
}

// 手写转换，练 Rust
fn to_base(mut n: i128, base: u32) -> String {
    if n == 0 {
        return "0".into();
    }

    let negative = n < 0;
    if negative {
        n = -n;
    }

    let mut digits = Vec::new();
    while n > 0 {
        let rem = (n % base as i128) as u32;
        digits.push(std::char::from_digit(rem, base).unwrap());
        n /= base as i128;
    }

    if negative {
        digits.push('-');
    }

    digits.iter().rev().collect()
}

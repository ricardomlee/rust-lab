use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HashRequest {
    pub input: String,
    pub algorithm: String,
}

#[derive(Serialize)]
pub struct HashResponse {
    pub input: String,
    pub algorithm: String,
    pub hash: String,
}

pub async fn compute(Json(req): Json<HashRequest>) -> Json<HashResponse> {
    let result = core::hash::hash_full(&req.input, &req.algorithm);
    Json(HashResponse {
        input: result.input,
        algorithm: result.algorithm,
        hash: result.hash,
    })
}

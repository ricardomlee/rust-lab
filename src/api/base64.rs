use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EncodeRequest {
    pub input: String,
    #[serde(default)]
    pub url_safe: bool,
}

#[derive(Serialize)]
pub struct Base64Response {
    pub value: String,
    pub original: String,
    pub operation: String,
    pub variant: String,
}

pub async fn encode(Json(req): Json<EncodeRequest>) -> Json<Base64Response> {
    let result = core::base64::encode_full(&req.input, req.url_safe);
    Json(Base64Response {
        value: result.value,
        original: result.original,
        operation: result.operation,
        variant: result.variant,
    })
}

#[derive(Deserialize)]
pub struct DecodeRequest {
    pub encoded: String,
    #[serde(default)]
    pub url_safe: bool,
}

pub async fn decode(
    Json(req): Json<DecodeRequest>,
) -> Result<Json<Base64Response>, (StatusCode, String)> {
    let result =
        core::base64::decode_full(&req.encoded, req.url_safe).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(Base64Response {
        value: result.value,
        original: result.original,
        operation: result.operation,
        variant: result.variant,
    }))
}

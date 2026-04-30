use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct UuidResponse {
    pub uuid: String,
    pub version: u8,
    pub variant: &'static str,
}

pub async fn generate() -> Json<UuidResponse> {
    Json(UuidResponse {
        uuid: core::uuid_gen::generate_uuid(),
        version: 4,
        variant: "RFC4122",
    })
}

#[derive(Serialize)]
pub struct ShortIdResponse {
    pub id: String,
    pub length: usize,
    pub url_safe: bool,
}

pub async fn short_id() -> Json<ShortIdResponse> {
    let id = core::uuid_gen::generate_short_id();
    Json(ShortIdResponse {
        length: id.len(),
        id,
        url_safe: true,
    })
}

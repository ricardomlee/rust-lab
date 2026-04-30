use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HexToRgbaRequest {
    pub hex: String,
}

#[derive(Serialize)]
pub struct HexToRgbaResponse {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub css: String,
}

pub async fn hex_to_rgba(
    Json(req): Json<HexToRgbaRequest>,
) -> Result<Json<HexToRgbaResponse>, (StatusCode, String)> {
    let (r, g, b, a) =
        core::color::hex_to_rgba(&req.hex).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(HexToRgbaResponse {
        r,
        g,
        b,
        a,
        css: core::color::format_rgba(r, g, b, a),
    }))
}

#[derive(Deserialize)]
pub struct RgbToHexRequest {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize)]
pub struct RgbToHexResponse {
    pub hex: String,
}

pub async fn rgb_to_hex(Json(req): Json<RgbToHexRequest>) -> Json<RgbToHexResponse> {
    Json(RgbToHexResponse {
        hex: core::color::rgb_to_hex(req.r, req.g, req.b),
    })
}

#[derive(Deserialize)]
pub struct RgbToHslRequest {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize)]
pub struct RgbToHslResponse {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub css: String,
}

pub async fn rgb_to_hsl(Json(req): Json<RgbToHslRequest>) -> Json<RgbToHslResponse> {
    let (h, s, l) = core::color::rgb_to_hsl(req.r, req.g, req.b);
    Json(RgbToHslResponse {
        h,
        s,
        l,
        css: core::color::format_hsl(h, s, l),
    })
}

#[derive(Deserialize)]
pub struct HslToRgbRequest {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

#[derive(Serialize)]
pub struct HslToRgbResponse {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
    pub css: String,
}

pub async fn hsl_to_rgb(Json(req): Json<HslToRgbRequest>) -> Json<HslToRgbResponse> {
    let (r, g, b) = core::color::hsl_to_rgb(req.h, req.s, req.l);
    Json(HslToRgbResponse {
        r,
        g,
        b,
        hex: core::color::rgb_to_hex(r, g, b),
        css: format!("rgb({}, {}, {})", r, g, b),
    })
}

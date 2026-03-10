use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyze_text(text: &str) -> Result<JsValue, JsValue> {
    let analysis = core::text::analyze(text).map_err(|e| JsValue::from_str(&e))?;
    Ok(serde_json::json!({
        "chars": analysis.chars,
        "words": analysis.words,
        "lines": analysis.lines,
        "sha256": analysis.sha256,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn base_convert(value: &str, from: u32, to: u32) -> Result<JsValue, JsValue> {
    let result = core::base::convert(value, from, to).map_err(|e| JsValue::from_str(&e))?;
    Ok(JsValue::from_str(&result))
}

#[wasm_bindgen]
pub fn format_bytes(bytes: u64) -> Result<JsValue, JsValue> {
    let formatted = core::bytes::format_byte_size(bytes);
    Ok(serde_json::json!({
        "bytes": formatted.bytes,
        "binary": formatted.binary,
        "decimal": formatted.decimal,
        "binary_per_second": formatted.binary_per_second,
        "decimal_per_second": formatted.decimal_per_second,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn format_duration(milliseconds: u64) -> Result<JsValue, JsValue> {
    let formatted = core::duration::format_duration(milliseconds);
    Ok(serde_json::json!({
        "milliseconds": formatted.milliseconds,
        "compact": formatted.compact,
        "clock": formatted.clock,
        "verbose_zh": formatted.verbose_zh,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn format_frame_time(fps: f64) -> Result<JsValue, JsValue> {
    let formatted = core::frame_time::format_frame_time(fps).map_err(|e| JsValue::from_str(&e))?;
    Ok(serde_json::json!({
        "fps": formatted.fps,
        "frame_ms": formatted.frame_ms,
        "label": formatted.label,
        "quality_hint": formatted.quality_hint,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn format_percent(value: f64) -> Result<JsValue, JsValue> {
    let formatted = core::percent::format_percent(value).map_err(|e| JsValue::from_str(&e))?;
    Ok(serde_json::json!({
        "value": formatted.value,
        "percent": formatted.percent,
        "ratio": formatted.ratio,
        "hint": formatted.hint,
    })
    .to_string()
    .into())
}


#[wasm_bindgen]
pub fn format_countdown(total_seconds: i64) -> Result<JsValue, JsValue> {
    let formatted = core::countdown::format_countdown(total_seconds);
    Ok(serde_json::json!({
        "total_seconds": formatted.total_seconds,
        "sign": formatted.sign,
        "days": formatted.days,
        "hours": formatted.hours,
        "minutes": formatted.minutes,
        "seconds": formatted.seconds,
        "clock": formatted.clock,
        "compact": formatted.compact,
        "status": formatted.status,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn hex_to_rgba(hex: &str) -> Result<JsValue, JsValue> {
    let (r, g, b, a) = core::color::hex_to_rgba(hex).map_err(|e| JsValue::from_str(&e))?;
    Ok(serde_json::json!({
        "r": r,
        "g": g,
        "b": b,
        "a": a,
        "css": core::color::format_rgba(r, g, b, a),
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> Result<JsValue, JsValue> {
    Ok(JsValue::from_str(&core::color::rgb_to_hex(r, g, b)))
}

#[wasm_bindgen]
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> Result<JsValue, JsValue> {
    let (h, s, l) = core::color::rgb_to_hsl(r, g, b);
    Ok(serde_json::json!({
        "h": h,
        "s": s,
        "l": l,
        "css": core::color::format_hsl(h, s, l),
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Result<JsValue, JsValue> {
    let (r, g, b) = core::color::hsl_to_rgb(h, s, l);
    Ok(serde_json::json!({
        "r": r,
        "g": g,
        "b": b,
        "hex": core::color::rgb_to_hex(r, g, b),
        "css": format!("rgb({}, {}, {})", r, g, b),
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn blend_colors(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8, ratio: f64) -> Result<JsValue, JsValue> {
    let (r, g, b) = core::color::blend_colors(r1, g1, b1, r2, g2, b2, ratio);
    Ok(serde_json::json!({
        "r": r,
        "g": g,
        "b": b,
        "hex": core::color::rgb_to_hex(r, g, b),
        "css": format!("rgb({}, {}, {})", r, g, b),
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn generate_palette(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8, steps: usize) -> Result<JsValue, JsValue> {
    let palette = core::color::generate_palette(r1, g1, b1, r2, g2, b2, steps);
    let colors: Vec<JsValue> = palette
        .iter()
        .map(|(r, g, b)| {
            serde_json::json!({
                "r": r,
                "g": g,
                "b": b,
                "hex": core::color::rgb_to_hex(*r, *g, *b),
            })
        })
        .map(|v| JsValue::from_str(&v.to_string()))
        .collect();
    Ok(JsValue::from(js_sys::Array::from_iter(colors)))
}

#[wasm_bindgen]
pub fn base64_encode(input: &str, url_safe: bool) -> Result<JsValue, JsValue> {
    let result = core::base64::encode_full(input, url_safe);
    Ok(serde_json::json!({
        "value": result.value,
        "original": result.original,
        "operation": result.operation,
        "variant": result.variant,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn base64_decode(encoded: &str, url_safe: bool) -> Result<JsValue, JsValue> {
    match core::base64::decode_full(encoded, url_safe) {
        Ok(result) => Ok(serde_json::json!({
            "value": result.value,
            "original": result.original,
            "operation": result.operation,
            "variant": result.variant,
        })
        .to_string()
        .into()),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

#[wasm_bindgen]
pub fn hash_compute(input: &str, algorithm: &str) -> Result<JsValue, JsValue> {
    let result = core::hash::hash_full(input, algorithm);
    Ok(serde_json::json!({
        "input": result.input,
        "algorithm": result.algorithm,
        "hash": result.hash,
    })
    .to_string()
    .into())
}

#[wasm_bindgen]
pub fn hash_verify(input: &str, expected_hash: &str, algorithm: &str) -> Result<JsValue, JsValue> {
    let valid = core::hash::verify_hash(input, expected_hash, algorithm);
    Ok(serde_json::json!({
        "input": input,
        "expected": expected_hash,
        "algorithm": algorithm,
        "valid": valid,
    })
    .to_string()
    .into())
}

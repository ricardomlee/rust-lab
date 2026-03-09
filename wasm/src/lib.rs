use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyze_text(text: &str) -> Result<JsValue, JsValue> {
    let analysis = core::text::analyze(text).map_err(JsValue::from_str)?;
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
    let result = core::base::convert(value, from, to).map_err(JsValue::from_str)?;
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
    let formatted = core::frame_time::format_frame_time(fps).map_err(JsValue::from_str)?;
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
    let formatted = core::percent::format_percent(value).map_err(JsValue::from_str)?;
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

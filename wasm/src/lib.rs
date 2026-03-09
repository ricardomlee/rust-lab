use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn base_convert(value: &str, from: u32, to: u32) -> Result<String, String> {
    core::base::convert(value, from, to)
}

#[wasm_bindgen]
pub fn analyze_text(text: &str) -> Result<JsValue, JsValue> {
    let analysis = core::text::analyze(text).map_err(JsValue::from_str)?;

    let json = format!(
        r#"{{"chars":{},"words":{},"lines":{},"sha256":"{}"}}"#,
        analysis.chars, analysis.words, analysis.lines, analysis.sha256
    );

    js_sys::JSON::parse(&json)
}

#[wasm_bindgen]
pub fn format_bytes(bytes: u64) -> Result<JsValue, JsValue> {
    let formatted = core::bytes::format_byte_size(bytes);

    let json = format!(
        r#"{{"bytes":{},"binary":"{}","decimal":"{}","binary_per_second":"{}","decimal_per_second":"{}"}}"#,
        formatted.bytes,
        formatted.binary,
        formatted.decimal,
        formatted.binary_per_second,
        formatted.decimal_per_second
    );

    js_sys::JSON::parse(&json)
}

#[wasm_bindgen]
pub fn format_duration(milliseconds: u64) -> Result<JsValue, JsValue> {
    let formatted = core::duration::format_duration(milliseconds);

    let json = format!(
        r#"{{"milliseconds":{},"compact":"{}","clock":"{}","verbose_zh":"{}"}}"#,
        formatted.milliseconds,
        formatted.compact,
        formatted.clock,
        formatted.verbose_zh
    );

    js_sys::JSON::parse(&json)
}

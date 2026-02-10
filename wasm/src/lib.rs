use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn base_convert(value: &str, from: u32, to: u32) -> Result<String, String> {
    core::base::convert(value, from, to)
}

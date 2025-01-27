use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn vec_to_js_array(vec: Vec<f64>) -> Result<JsValue, JsValue> {
    // Use serde-wasm-bindgen to serialize the Vec<f64> into a JsValue
    Ok(serde_wasm_bindgen::to_value(&vec)?)
}
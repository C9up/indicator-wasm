use wasm_bindgen::JsValue;

pub fn get_f64_from_jsvalue(value: &JsValue) -> f64 {
    value.as_f64().unwrap_or(f64::NAN)
}
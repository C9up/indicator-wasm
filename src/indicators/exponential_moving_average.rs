use crate::calculate_ema_helper::calculate_ema;
use crate::{create_error, jsvalue_to_f64, serialize_to_js_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exponential_moving_average(data: JsValue, period: usize) -> Result<JsValue, JsValue> {
    let vec_data = jsvalue_to_f64(data);

    if vec_data.is_empty() {
        return Err(create_error("Prices vector must not be empty."));
    }

    let result = calculate_ema(&vec_data, period);
    Ok(serialize_to_js_value(&result)?)
}

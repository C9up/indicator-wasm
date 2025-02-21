use serde::de::DeserializeOwned;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod bollinger_bands_helper;
pub mod calculate_atr_helper;
pub mod calculate_ema_helper;
pub mod calculate_sma_helper;
pub mod directional_movement_index_helper;
pub mod entry_exit_signals_helper;
pub mod highest_lowest_helper;
pub mod ichimoku_helper;
pub mod low_high_open_close_volume_date_to_array_helper;
pub mod pivot_points_helper;
pub mod relative_strength_index_helper;
pub mod smooth_helper;
pub mod stochastic_momentum_index_helper;
pub mod vec_to_js_array_helper;

pub fn deserialize_js_value<T: DeserializeOwned>(data: &JsValue) -> Result<T, JsValue> {
    from_value(data.clone())
        .map_err(|e| JsValue::from_str(&format!("Error deserializing input: {:?}", e)))
}

pub fn serialize_to_js_value<T>(value: &T) -> Result<wasm_bindgen::JsValue, JsValue>
where
    T: ?Sized + serde::ser::Serialize,
{
    to_value(value).map_err(|e| JsValue::from_str(&format!("Error serializing result: {:?}", e)))
}

pub fn jsvalue_to_f64(js_value: JsValue) -> Vec<f64> {
    from_value::<Vec<f64>>(js_value).unwrap()
}

#[wasm_bindgen(inline_js = "
export function create_error(msg) {
    return new Error(msg);
}
")]
extern "C" {
    pub fn create_error(msg: &str) -> JsValue;
}

#[wasm_bindgen(inline_js = "
export function rust_console_log(msg) {
    console.log('[DEBUG]', msg);
    return msg; // Optionally, return the logged message.
}
")]
extern "C" {
    /// Logs a message to the browser console and returns it.
    pub fn rust_console_log(msg: &str) -> String;
}

use wasm_bindgen::prelude::*;
use crate::{create_error, deserialize_js_value, serialize_to_js_value};
use crate::helpers::bollinger_bands_helper::compute_bollinger_bands;

/// WASM-exposed function that calculates Bollinger Bands.
///
/// - `data`: JS array (`number[]`) containing price values.
/// - `period`: number of periods for the moving average.
/// - `multiplier`: coefficient (often 2) for calculating the upper and lower bands.
///
/// Returns a serialized JS object containing three arrays of numbers.
#[wasm_bindgen]
pub fn bollinger_bands(
    data: &JsValue,
    period: Option<usize>,
    multiplier: Option<f64>
) -> Result<JsValue, JsValue> {

    // Default values
    let period = period.unwrap_or(20);
    let multiplier = multiplier.unwrap_or(2f64);
    let prices: Vec<f64> = deserialize_js_value(data)?;

    if period <= 0 {
        return Err(create_error("Period must be greater than 0."));
    }

    if multiplier <= 0.0 {
        return Err(create_error("Multiplier must be greater than 0."));
    }

    // Validate input: prices vector must not be empty
    if prices.is_empty() {
        return Err(create_error("Prices vector must not be empty."));
    }

    // Compute Bollinger Bands using the helper function
    let result = compute_bollinger_bands(&prices, period, multiplier);

    // Serialize the result into JsValue (a JS object with 3 properties containing number[])
    Ok(serialize_to_js_value(&result)?)
}

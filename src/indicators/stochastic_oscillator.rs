use wasm_bindgen::prelude::*;
use crate::deserialize_js_value;
use crate::highest_lowest_helper::calculate_high_low;
use crate::low_high_open_close_volume_date_to_array_helper::{low_high_open_close_volume_date_to_array, MarketDataResult};

#[wasm_bindgen]
pub fn stochastic_oscillator(
    data: JsValue,
    period: usize
)  -> Result<Vec<f64>, JsValue> {

    let processed_data = low_high_open_close_volume_date_to_array(data)?;
    let market_data: MarketDataResult = deserialize_js_value(&processed_data)?;

    let highs = market_data.highs;
    let lows = market_data.lows;
    let closes = market_data.closes;

    let mut result = Vec::with_capacity(closes.len());

    for i in period..closes.len() {
        let (highest_high, lowest_low) = calculate_high_low(&highs, &lows, i - period, i - 1);
        let stoch = 100.0 * (closes[i] - lowest_low) / (highest_high - lowest_low);
        result.push(stoch);
    }

    Ok(result)
}

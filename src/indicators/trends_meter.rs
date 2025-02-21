use crate::calculate_ema_helper::calculate_ema;
use crate::deserialize_js_value;
use crate::helpers::directional_movement_index_helper::true_range;
use crate::low_high_open_close_volume_date_to_array_helper::{
    low_high_open_close_volume_date_to_array, MarketDataResult,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn trends_meter(data: JsValue, period: Option<usize>) -> Result<Vec<f64>, JsValue> {
    let period = period.unwrap_or(14);
    if period < 2 {
        return Err(JsValue::from_str("The period must be greater than 1"));
    }

    let processed_data = low_high_open_close_volume_date_to_array(data)?;
    let market_data: MarketDataResult = deserialize_js_value(&processed_data)?;

    let highs = market_data.highs;
    let lows = market_data.lows;
    let closes = market_data.closes;

    if closes.len() < period {
        return Err(JsValue::from_str(
            "Not enough data for the specified period",
        ));
    }

    let mut tr = vec![0.0; closes.len()];
    for i in 1..closes.len() {
        tr[i] = true_range(&highs, &lows, &closes, i);
    }

    let tr_ema = calculate_ema(&tr, period);

    let mut momentum = vec![0.0; closes.len()];
    for i in period..closes.len() {
        momentum[i] = closes[i] - closes[i - period];
    }

    let momentum_ema = calculate_ema(&momentum, period);

    let mut trends_meter = vec![0.0; closes.len()];
    for i in period..closes.len() {
        trends_meter[i] = (tr_ema[i] + momentum_ema[i]) / 2.0;
    }

    Ok(trends_meter)
}

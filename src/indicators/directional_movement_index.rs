use wasm_bindgen::prelude::*;
use crate::{create_error, deserialize_js_value};
use crate::helpers::directional_movement_index_helper::{directional_movement, true_range};
use crate::low_high_open_close_volume_date_to_array_helper::{low_high_open_close_volume_date_to_array, MarketDataResult};

// Main DMI function
#[wasm_bindgen]
pub fn directional_movement_index(
    data: JsValue,
    period: usize
) -> Result<Vec<f64>, JsValue> {

    if period <= 0 {
        return Err(create_error("Period must be greater than 0."));
    }

    let processed_data = low_high_open_close_volume_date_to_array(data)?;
    let market_data: MarketDataResult = deserialize_js_value(&processed_data)?;

    let highs = market_data.highs;
    let lows = market_data.lows;
    let closes = market_data.closes;

    let len = highs.len();
    if len < period {
        return Err(create_error("Not enough data points"));
    }

    let mut plus_di = vec![0.0; len];
    let mut minus_di = vec![0.0; len];
    let mut adx = vec![0.0; len];

    let mut tr_values = vec![0.0; len];
    let mut plus_dm = vec![0.0; len];
    let mut minus_dm = vec![0.0; len];

    // Calculate true range and directional movements
    for i in 1..len {
        tr_values[i] = true_range(&highs, &lows, &closes, i);
        let (p_dm, m_dm) = directional_movement(&highs, &lows, i);
        plus_dm[i] = p_dm;
        minus_dm[i] = m_dm;
    }

    // Calculate DI and ADX
    for i in period..len {
        let plus_dm_sum: f64 = plus_dm[i - period..i].iter().sum();
        let minus_dm_sum: f64 = minus_dm[i - period..i].iter().sum();
        let tr_sum: f64 = tr_values[i - period..i].iter().sum();

        if tr_sum != 0.0 {
            plus_di[i] = (plus_dm_sum / tr_sum) * 100.0;
            minus_di[i] = (minus_dm_sum / tr_sum) * 100.0;
        }

        let di_sum = plus_di[i] + minus_di[i];
        if di_sum != 0.0 {
            let di_diff = (plus_di[i] - minus_di[i]).abs();
            adx[i] = (di_diff / di_sum) * 100.0;
        }
    }

    Ok(adx)
}

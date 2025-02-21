use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::{deserialize_js_value, MarketDataResult};
use crate::calculate_ema_helper::calculate_ema;
use crate::highest_lowest_helper::calculate_high_low;
use crate::low_high_open_close_volume_date_to_array_helper::low_high_open_close_volume_date_to_array;

/// Calculate the Stochastic Momentum Index (SMI).
///
/// The SMI is computed using the following steps:
///
/// For each valid index i (i >= lookback - 1):
///   1. Let HH be the highest high over the last `lookback` periods.
///   2. Let LL be the lowest low over the last `lookback` periods.
///   3. Calculate the midpoint M = (HH + LL) / 2.
///   4. Calculate D = Close[i] - M.
///   5. Calculate R = HH - LL.
///
/// Then, apply double EMA smoothing:
///   - First, compute EMA(D, lookback) and EMA(R, lookback).
///   - Next, compute the EMA of the previous EMA results using the smoothing period:
///         D' = EMA( EMA(D, lookback), smoothing )
///         R' = EMA( EMA(R, lookback), smoothing )
///
/// Finally, the SMI is calculated as:
///         SMI = (D' / (0.5 * R')) * 100
///
/// For indices where there is insufficient data, the SMI value is NaN.
#[wasm_bindgen]
pub fn stochastic_momentum_index(
    data: JsValue,
    period_k: Option<usize>, // Look-back period for high/low calculation (default 14)
    period_d: Option<usize>  // Smoothing period for the second EMA (default 3)
) -> Result<Vec<f64>, JsValue> {
    // Deserialize and process the input data.
    let processed_data = low_high_open_close_volume_date_to_array(data)?;
    let market_data: MarketDataResult = deserialize_js_value(&processed_data)?;

    let n = market_data.highs.len();
    let lookback = period_k.unwrap_or(14);
    let smoothing = period_d.unwrap_or(3);

    // If there is not enough data to form one full look-back window, return all NaNs.
    if n < lookback {
        return Ok(vec![f64::NAN; n]);
    }

    // Prepare vectors for D (diff) and R (range) for valid indices.
    // These arrays correspond to indices from (lookback - 1) to (n - 1).
    let valid_len = n - lookback + 1;
    let mut diff = Vec::with_capacity(valid_len);
    let mut range = Vec::with_capacity(valid_len);

    // For each valid index i, compute HH and LL using the provided helper.
    // The window for index i is from (i - lookback + 1) to i.
    for i in (lookback - 1)..n {
        let start = i - lookback + 1;
        let (hh, ll) = calculate_high_low(&market_data.highs, &market_data.lows, start, i);
        let midpoint = (hh + ll) / 2.0;
        diff.push(market_data.closes[i] - midpoint);
        range.push(hh - ll);
    }

    // First level of EMA smoothing using the look-back period.
    let ema1 = calculate_ema(&diff, lookback);
    let ema2 = calculate_ema(&ema1, smoothing);
    let ema3 = calculate_ema(&range, lookback);
    let ema4 = calculate_ema(&ema3, smoothing);

    let mut smi = vec![f64::NAN; lookback - 1];
    for i in 0..ema2.len() {
        smi.push(100.0 * (ema2[i] / ema4[i]));
    }

    Ok(smi)
}

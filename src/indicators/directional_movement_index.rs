use wasm_bindgen::prelude::*;
use crate::low_high_open_close_volume_date_to_array::{low_high_open_close_volume_date_deserialize, low_high_open_close_volume_date_to_array};
use crate::smooth::smooth;

/// Represents the result of the Directional Movement Index (DMI) calculation.
#[wasm_bindgen]
pub struct DMIResult {
    plus_di: Vec<f64>,
    minus_di: Vec<f64>,
    adx: Vec<f64>,
}

#[wasm_bindgen]
impl DMIResult {
    /// Returns the +DI values as a vector of f64.
    #[wasm_bindgen(getter)]
    pub fn plus_di(&self) -> Vec<f64> {
        self.plus_di.clone()
    }

    /// Returns the -DI values as a vector of f64.
    #[wasm_bindgen(getter)]
    pub fn minus_di(&self) -> Vec<f64> {
        self.minus_di.clone()
    }

    /// Returns the ADX values as a vector of f64.
    #[wasm_bindgen(getter)]
    pub fn adx(&self) -> Vec<f64> {
        self.adx.clone()
    }
}

/// Represents the Directional Movement Index (DMI) calculator.
#[wasm_bindgen]
pub struct DirectionalMovementIndex {
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
}

#[wasm_bindgen]
impl DirectionalMovementIndex {
    /// Creates a new `DirectionalMovementIndex` instance from a JavaScript object containing price data.
    ///
    /// # Arguments
    /// * `prices` - A JavaScript object containing `highs`, `lows`, and `closes` arrays.
    ///
    /// # Errors
    /// Returns an error if the input data is invalid (e.g., empty arrays or mismatched lengths).
    #[wasm_bindgen(constructor)]
    pub fn new(prices: JsValue) -> Result<DirectionalMovementIndex, JsValue> {
        let segment = low_high_open_close_volume_date_deserialize(low_high_open_close_volume_date_to_array(prices)
            .expect("Failed to convert market data"));

        // Validate input data
        if segment.highs.is_empty() || segment.lows.is_empty() || segment.closes.is_empty() {
            return Err(JsValue::from_str("Price arrays cannot be empty."));
        }
        if segment.highs.len() != segment.lows.len() || segment.highs.len() != segment.closes.len() {
            return Err(JsValue::from_str("Price arrays must have the same length."));
        }

        Ok(DirectionalMovementIndex {
            highs: segment.highs,
            lows: segment.lows,
            closes: segment.closes,
        })
    }

    /// Calculates the Directional Movement Index (DMI) for the given period.
    ///
    /// # Arguments
    /// * `period` - The smoothing period for the DMI calculation.
    ///
    /// # Errors
    /// Returns an error if the period is greater than the length of the price data.
    pub fn period(&self, period: usize) -> Result<DMIResult, JsValue> {
        let len = self.highs.len();
        if len < period {
            return Err(JsValue::from_str("Period cannot be greater than the length of the price data."));
        }

        // Calculate True Range (TR), +DM, and -DM
        let (tr, plus_dm, minus_dm) = (0..len)
            .map(|i| {
                if i == 0 {
                    (0.0, 0.0, 0.0)
                } else {
                    let high = self.highs[i];
                    let low = self.lows[i];
                    let prev_close = self.closes[i - 1];

                    let tr = (high - low).max((high - prev_close).abs()).max((low - prev_close).abs());

                    let up_move = high - self.highs[i - 1];
                    let down_move = self.lows[i - 1] - low;

                    let plus_dm = if up_move > down_move && up_move > 0.0 {
                        up_move
                    } else {
                        0.0
                    };

                    let minus_dm = if down_move > up_move && down_move > 0.0 {
                        down_move
                    } else {
                        0.0
                    };

                    (tr, plus_dm, minus_dm)
                }
            })
            .fold(
                (Vec::with_capacity(len), Vec::with_capacity(len), Vec::with_capacity(len)),
                |(mut tr, mut plus_dm, mut minus_dm), (tr_val, plus_dm_val, minus_dm_val)| {
                    tr.push(tr_val);
                    plus_dm.push(plus_dm_val);
                    minus_dm.push(minus_dm_val);
                    (tr, plus_dm, minus_dm)
                },
            );

        // Smooth +DM, -DM, and TR
        let smoothed_plus_dm = smooth(&plus_dm, period);
        let smoothed_minus_dm = smooth(&minus_dm, period);
        let smoothed_tr = smooth(&tr, period);

        // Calculate +DI and -DI
        let (plus_di, minus_di): (Vec<f64>, Vec<f64>) = (0..len)
            .map(|i| {
                if smoothed_tr[i] > 0.0 {
                    (
                        100.0 * (smoothed_plus_dm[i] / smoothed_tr[i]),
                        100.0 * (smoothed_minus_dm[i] / smoothed_tr[i]),
                    )
                } else {
                    (0.0, 0.0)
                }
            })
            .unzip();

        // Calculate ADX
        let adx = smooth(
            &(period..len)
                .map(|i| {
                    if plus_di[i] + minus_di[i] > 0.0 {
                        ((plus_di[i] - minus_di[i]).abs() / (plus_di[i] + minus_di[i])) * 100.0
                    } else {
                        0.0
                    }
                })
                .collect::<Vec<f64>>(),
            period,
        );

        Ok(DMIResult {
            plus_di,
            minus_di,
            adx,
        })
    }
}
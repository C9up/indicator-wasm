use wasm_bindgen::prelude::*;
use crate::calculate_ema::calculate_ema;
use crate::low_high_open_close_volume_date_to_array::{low_high_open_close_volume_date_deserialize, low_high_open_close_volume_date_to_array};

#[wasm_bindgen]
pub struct StochasticMomentumIndex {
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
}

#[wasm_bindgen]
impl StochasticMomentumIndex {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: JsValue) -> Self {
        let segment = low_high_open_close_volume_date_deserialize(low_high_open_close_volume_date_to_array(prices)
            .expect("Failed to convert market data"));
        StochasticMomentumIndex {
            highs: segment.highs,
            lows: segment.lows,
            closes: segment.closes,
        }
    }
    pub fn period(
        &self,
        period_l: usize,
        period_h: usize,
        smoothing_period: usize,
    ) -> Vec<f64> {
        let len = self.highs.len();
        let mut smi_values = Vec::with_capacity(len);

        for i in 0..len {
            // Check if there is enough data to apply the periods
            if i < period_h - 1 || i < period_l - 1 {
                smi_values.push(f64::NAN);
            } else {
                // Define the starting indices for the sliding windows
                let start_h = if i + 1 < period_h { 0 } else { i + 1 - period_h };
                let start_l = if i + 1 < period_l { 0 } else { i + 1 - period_l };

                // Calculate the highest value over period_h and the lowest value over period_l
                let highest = self.highs[start_h..=i]
                    .iter()
                    .cloned()
                    .fold(f64::NEG_INFINITY, f64::max);
                let lowest = self.lows[start_l..=i]
                    .iter()
                    .cloned()
                    .fold(f64::INFINITY, f64::min);

                // Compute the midpoint, the difference, and the range
                let midpoint = (highest + lowest) / 2.0;
                let diff = self.closes[i] - midpoint;
                let range = highest - lowest;

                // Calculate the SMI value, avoiding division by zero
                let value = if range == 0.0 {
                    0.0
                } else {
                    (diff / (range / 2.0)) * 100.0
                };
                smi_values.push(value);
            }
        }
        // Apply EMA smoothing to the computed SMI series
        calculate_ema(&smi_values, smoothing_period)
    }
}
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
    pub fn calculate(&self, period: usize, smoothing_period: usize) -> Vec<f64> {
        let len = self.highs.len();

        // Precompute high - low ranges to avoid redundant calculations
        let high_low_ranges: Vec<f64> = self.highs.iter()
            .zip(&self.lows)
            .map(|(&high, &low)| high - low)
            .collect();

        // Calculate midpoints and differences in a single pass
        let (midpoints, diffs): (Vec<f64>, Vec<f64>) = self.highs.iter()
            .zip(&self.lows)
            .zip(&self.closes)
            .map(|((&high, &low), &close)| {
                let midpoint = (high + low) / 2.0;
                let diff = close - midpoint;
                (midpoint, diff)
            })
            .unzip();

        // Smooth the diffs and high-low ranges
        let smoothed_diffs = calculate_ema(&diffs, smoothing_period); // Pas de gestion d'erreur
        let smoothed_ranges = calculate_ema(&high_low_ranges, smoothing_period); // Pas de gestion d'erreur

        // Calculate SMI
        let smi_values: Vec<f64> = smoothed_diffs.iter()
            .zip(&smoothed_ranges)
            .map(|(&diff, &range)| {
                if range == 0.0 {
                    0.0 // Avoid division by zero
                } else {
                    (diff / (range / 2.0)) * 100.0
                }
            })
            .collect();

        smi_values
    }
}
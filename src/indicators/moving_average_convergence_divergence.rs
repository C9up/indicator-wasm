use wasm_bindgen::prelude::*;
use crate::utils;

#[wasm_bindgen]
pub struct MovingAverageConvergenceDivergence {
    prices: Vec<f64>,
}

#[wasm_bindgen]
pub struct MACDResult {
    #[wasm_bindgen(getter_with_clone)]
    pub histogram: Vec<f64>,
    #[wasm_bindgen(getter_with_clone)]
    pub signal_line: Vec<f64>,
    #[wasm_bindgen(getter_with_clone)]
    pub macd_line: Vec<f64>,
}

#[wasm_bindgen]
impl MovingAverageConvergenceDivergence {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        MovingAverageConvergenceDivergence {
            prices
        }
    }

    pub fn period(&mut self, fast_period: usize, slow_period: usize, signal_period: usize) -> MACDResult {
        let mut macd_line = Vec::new();
        let mut signal_line = Vec::new();
        let mut histogram = Vec::new();

        let len = self.prices.len();
        if len < fast_period || len < slow_period || len < signal_period {
            return MACDResult {
                histogram: vec![],
                signal_line: vec![],
                macd_line: vec![],
            };
        }

        // Calculate the fast and slow EMAs
        let fast_ema = utils::calculate_ema(&self.prices, fast_period);
        let slow_ema = utils::calculate_ema(&self.prices, slow_period);

        // Calculate the MACD Line
        for i in 0..fast_ema.len() {
            macd_line.push(fast_ema[i] - slow_ema[i]);
        }

        // Calculate the Signal Line (EMA of the MACD Line)
        signal_line = utils::calculate_ema(&macd_line, signal_period);

        // Calculate the Histogram
        for i in 0..macd_line.len() {
            histogram.push(macd_line[i] - signal_line[i]);
        }

        MACDResult {
            histogram,
            signal_line,
            macd_line,
        }
    }
}
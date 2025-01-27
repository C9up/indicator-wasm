use wasm_bindgen::prelude::*;
use crate::calculate_ema::calculate_ema;

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
        let len = self.prices.len();
        if len < fast_period || len < slow_period || len < signal_period {
            return MACDResult {
                histogram: vec![],
                signal_line: vec![],
                macd_line: vec![],
            };
        }

        // Calculate the fast and slow EMAs
        let fast_ema = calculate_ema(&self.prices, fast_period);
        let slow_ema = calculate_ema(&self.prices, slow_period);

        // Pre-allocate memory for the MACD line
        let mut macd_line = Vec::with_capacity(fast_ema.len());

        // Calculate the MACD Line
        for (f, s) in fast_ema.iter().zip(&slow_ema) {
            macd_line.push(f - s);
        }

        // Calculate the Signal Line (EMA of the MACD Line)
        let signal_line = calculate_ema(&macd_line, signal_period);

        // Pre-allocate memory for the histogram
        let mut histogram = Vec::with_capacity(macd_line.len());

        // Calculate the Histogram
        for (m, s) in macd_line.iter().zip(&signal_line) {
            histogram.push(m - s);
        }

        MACDResult {
            histogram,
            signal_line,
            macd_line,
        }
    }
}
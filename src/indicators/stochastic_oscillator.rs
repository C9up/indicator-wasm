use wasm_bindgen::prelude::*;
use crate::utils;

#[wasm_bindgen]
pub struct StochasticOscillatorResult {
    percent_k: Vec<f64>,
    percent_d: Vec<f64>,
}

#[wasm_bindgen]
impl StochasticOscillatorResult {
    #[wasm_bindgen(getter)]
    pub fn percent_k(&self) -> Vec<f64> {
        self.percent_k.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn percent_d(&self) -> Vec<f64> {
        self.percent_d.clone()
    }
}

#[wasm_bindgen]
pub struct StochasticOscillator {
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
}

#[wasm_bindgen]
impl StochasticOscillator {

    #[wasm_bindgen(constructor)]
    pub fn new(highs: Vec<f64>, lows: Vec<f64>, closes: Vec<f64>) -> Self {
        StochasticOscillator {
            highs,
            lows,
            closes,
        }
    }

    pub fn period(&mut self, period: usize, smoothing_period: usize) -> StochasticOscillatorResult {
        let mut percent_k = Vec::new();
        let mut percent_d = Vec::new();

        let len = self.highs.len();
        if len < period || len < smoothing_period {
            return StochasticOscillatorResult {
                percent_k: vec![],
                percent_d: vec![],
            };
        }

        // Calculate %K
        for i in (period - 1)..len {
            let highest_high = self.highs[(i - period + 1)..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.lows[(i - period + 1)..=i].iter().cloned().fold(f64::MAX, f64::min);

            let close = self.closes[i];
            let percent_k_result = ((close - lowest_low) / (highest_high - lowest_low)) * 100.0;
            percent_k.push(percent_k_result);
        }

        // Calculate %D (SMA of %K)
        percent_d = utils::calculate_sma(&percent_k, smoothing_period);

        StochasticOscillatorResult {
            percent_k,
            percent_d,
        }
    }
}
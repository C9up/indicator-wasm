use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use crate::calculate_sma::calculate_sma;

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
        StochasticOscillator { highs, lows, closes }
    }

    pub fn period(&self, period: usize, smoothing_period: usize) -> StochasticOscillatorResult {
        let len = self.highs.len();
        if len < period || len < smoothing_period {
            return StochasticOscillatorResult {
                percent_k: vec![],
                percent_d: vec![],
            };
        }

        let mut percent_k = Vec::with_capacity(len - period + 1);
        let mut max_deque = VecDeque::with_capacity(period);
        let mut min_deque = VecDeque::with_capacity(period);

        // Calculate %K using sliding window for max and min
        for i in 0..len {
            // Maintain the deque for max values
            while !max_deque.is_empty() && self.highs[*max_deque.back().unwrap()] <= self.highs[i] {
                max_deque.pop_back();
            }
            max_deque.push_back(i);

            // Maintain the deque for min values
            while !min_deque.is_empty() && self.lows[*min_deque.back().unwrap()] >= self.lows[i] {
                min_deque.pop_back();
            }
            min_deque.push_back(i);

            // Remove elements outside the current window
            if *max_deque.front().unwrap() <= i - period {
                max_deque.pop_front();
            }
            if *min_deque.front().unwrap() <= i - period {
                min_deque.pop_front();
            }

            // Calculate %K if the window is complete
            if i >= period - 1 {
                let highest_high = self.highs[*max_deque.front().unwrap()];
                let lowest_low = self.lows[*min_deque.front().unwrap()];
                let close = self.closes[i];
                let k_value = ((close - lowest_low) / (highest_high - lowest_low)) * 100.0;
                percent_k.push(k_value);
            }
        }

        // Calculate %D (SMA of %K)
        let percent_d = calculate_sma(&percent_k, smoothing_period);

        StochasticOscillatorResult {
            percent_k,
            percent_d,
        }
    }
}
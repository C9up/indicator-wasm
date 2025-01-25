use wasm_bindgen::prelude::*;
use crate::utils;

#[wasm_bindgen]
pub struct StochasticOscillator {
    period: usize,
    smoothing_period: usize,
    high_prices: Vec<f64>,
    low_prices: Vec<f64>,
    close_prices: Vec<f64>,
    percent_k: Vec<f64>,
    percent_d: Vec<f64>,
}

#[wasm_bindgen]
impl StochasticOscillator {
    pub fn new(period: usize, smoothing_period: usize, high_prices: Vec<f64>, low_prices: Vec<f64>, close_prices: Vec<f64>) -> Self {
        StochasticOscillator {
            period,
            smoothing_period,
            high_prices,
            low_prices,
            close_prices,
            percent_k: Vec::new(),
            percent_d: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.high_prices.len();
        if len < self.period || len < self.smoothing_period {
            return; // Not enough data to calculate Stochastic Oscillator
        }

        // Calculate %K
        for i in (self.period - 1)..len {
            let highest_high = self.high_prices[(i - self.period + 1)..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[(i - self.period + 1)..=i].iter().cloned().fold(f64::MAX, f64::min);

            let close = self.close_prices[i];
            let percent_k = ((close - lowest_low) / (highest_high - lowest_low)) * 100.0;
            self.percent_k.push(percent_k);
        }

        // Calculate %D (SMA of %K)
        self.percent_d = utils::calculate_sma(&self.percent_k, self.smoothing_period);
    }

    pub fn get_percent_k(&self) -> Vec<f64> {
        self.percent_k.clone()
    }

    pub fn get_percent_d(&self) -> Vec<f64> {
        self.percent_d.clone()
    }
}
// src/indicator/utils
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExponentialMovingAverage {
    prices: Vec<f64>
}

#[wasm_bindgen]
impl ExponentialMovingAverage {
    pub fn new(prices: Vec<f64>) -> Self {
        ExponentialMovingAverage {
            prices
        }
    }
    pub fn period(&mut self, period: usize) -> Vec<f64> {
        let len = self.prices.len();
        let mut ema_values = Vec::new();
        if len < period {
            return vec![];
        }

        let smoothing_factor = 2.0 / (period as f64 + 1.0);

        // Calculate the initial SMA (Simple Moving Average) as the first EMA value
        let mut sma = 0.0;
        for i in 0..period {
            sma += self.prices[i];
        }
        sma /= period as f64;

        ema_values.push(sma);

        // Calculate EMA for the remaining prices
        for i in period..len {
            let ema_today = (self.prices[i] * smoothing_factor) + (ema_values[i - period] * (1.0 - smoothing_factor));
            ema_values.push(ema_today);
        }

        ema_values
    }

}
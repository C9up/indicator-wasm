// src/indicator/utils
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExponentialMovingAverage {
    period: usize,
    prices: Vec<f64>,
    ema_values: Vec<f64>,
}

#[wasm_bindgen]
impl ExponentialMovingAverage {
    pub fn new(period: usize, prices: Vec<f64>) -> Self {
        ExponentialMovingAverage {
            period,
            prices,
            ema_values: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.prices.len();
        if len < self.period {
            return;
        }

        let smoothing_factor = 2.0 / (self.period as f64 + 1.0);

        // Calculate the initial SMA (Simple Moving Average) as the first EMA value
        let mut sma = 0.0;
        for i in 0..self.period {
            sma += self.prices[i];
        }
        sma /= self.period as f64;

        self.ema_values.push(sma);

        // Calculate EMA for the remaining prices
        for i in self.period..len {
            let ema_today = (self.prices[i] * smoothing_factor) + (self.ema_values[i - self.period] * (1.0 - smoothing_factor));
            self.ema_values.push(ema_today);
        }
    }

    pub fn get_ema_values(&self) -> Vec<f64> {
        self.ema_values.clone()
    }
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExponentialMovingAverage {
    prices: Vec<f64>, // Stores the price data for EMA calculation
}

#[wasm_bindgen]
impl ExponentialMovingAverage {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        ExponentialMovingAverage { prices } // Initialize the struct with the provided prices
    }

    pub fn period(&mut self, period: usize) -> Vec<f64> {
        // Handle edge cases: return an empty array if the period is 0 or the data is empty
        if period == 0 || self.prices.is_empty() {
            return Vec::new();
        }

        let len = self.prices.len();
        let mut ema_values = Vec::new(); // Vector to store the calculated EMA values

        // If the data length is less than the period, return an empty array
        if len < period {
            return Vec::new();
        }

        // Calculate the smoothing factor for EMA
        let smoothing_factor = 2.0 / (period as f64 + 1.0);

        // Calculate the initial SMA (Simple Moving Average) as the first EMA value
        let mut sma = 0.0;
        for i in 0..period {
            sma += self.prices[i];
        }
        sma /= period as f64;

        ema_values.push(sma); // Add the initial SMA value to the EMA vector

        // Calculate EMA for the remaining prices
        for i in period..len {
            let ema_today = (self.prices[i] * smoothing_factor) + (ema_values[i - period] * (1.0 - smoothing_factor));
            ema_values.push(ema_today); // Add the calculated EMA value to the vector
        }

        ema_values // Return the vector of EMA values
    }
}
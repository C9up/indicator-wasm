use wasm_bindgen::prelude::*;
use crate::smooth::smooth;

#[wasm_bindgen]
pub struct RelativeStrengthIndex {
    prices: Vec<f64>,
}

#[wasm_bindgen]
impl RelativeStrengthIndex {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        RelativeStrengthIndex { prices }
    }

    /// Calculate the RSI for a given period
    pub fn period(&self, period: usize) -> Vec<f64> {
        let len = self.prices.len();
        if len < period {
            return Vec::new(); // Not enough data to calculate RSI
        }

        let mut rsi_values = Vec::with_capacity(len - period);
        let mut avg_gain = 0.0;
        let mut avg_loss = 0.0;

        // Calculate initial average gains and losses
        for i in 1..=period {
            let change = self.prices[i] - self.prices[i - 1];
            if change > 0.0 {
                avg_gain += change;
            } else {
                avg_loss += -change;
            }
        }

        avg_gain /= period as f64;
        avg_loss /= period as f64;

        // Calculate the first RSI value
        let rs = if avg_loss == 0.0 {
            100.0
        } else {
            avg_gain / avg_loss
        };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsi_values.push(rsi);

        // Calculate RSI for the remaining data using an incremental approach
        for i in (period + 1)..len {
            let change = self.prices[i] - self.prices[i - 1];
            let (gain, loss) = if change > 0.0 {
                (change, 0.0)
            } else {
                (0.0, -change)
            };

            // Update average gains and losses incrementally
            avg_gain = ((avg_gain * (period - 1) as f64) + gain) / period as f64;
            avg_loss = ((avg_loss * (period - 1) as f64) + loss) / period as f64;

            // Calculate RSI
            let rs = if avg_loss == 0.0 {
                100.0
            } else {
                avg_gain / avg_loss
            };
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            rsi_values.push(rsi);
        }

        rsi_values
    }

    /// Calculate the RSI and apply smoothing if necessary
    pub fn period_smoothed(&self, period: usize, smoothing_period: usize) -> Vec<f64> {
        let rsi_values = self.period(period); // Calculate RSI

        // Apply smoothing if smoothing_period > 0
        if smoothing_period > 0 {
            return smooth(&rsi_values, smoothing_period);
        }

        rsi_values
    }
}
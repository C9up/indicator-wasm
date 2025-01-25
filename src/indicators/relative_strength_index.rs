use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RelativeStrengthIndex {
    period: usize,
    prices: Vec<f64>,
    rsi_values: Vec<f64>,
}

#[wasm_bindgen]
impl RelativeStrengthIndex {
    pub fn new(period: usize, prices: Vec<f64>) -> Self {
        RelativeStrengthIndex {
            period,
            prices,
            rsi_values: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.prices.len();
        if len < self.period {
            return; // Not enough data to calculate RSI
        }

        let mut gains = Vec::new();
        let mut losses = Vec::new();

        // Calculate initial gains and losses
        for i in 1..len {
            let change = self.prices[i] - self.prices[i - 1];
            if change > 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-change);
            }
        }

        // Calculate initial average gain and average loss
        let mut avg_gain: f64 = gains[0..self.period].iter().sum::<f64>() / self.period as f64;
        let mut avg_loss: f64 = losses[0..self.period].iter().sum::<f64>() / self.period as f64;

        // Calculate initial RSI
        let rs = if avg_loss == 0.0 {
            100.0
        } else {
            avg_gain / avg_loss
        };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        self.rsi_values.push(rsi);

        // Calculate RSI for the remaining prices
        for i in self.period..(len - 1) {
            avg_gain = ((avg_gain * (self.period - 1) as f64) + gains[i]) / self.period as f64;
            avg_loss = ((avg_loss * (self.period - 1) as f64) + losses[i]) / self.period as f64;

            let rs = if avg_loss == 0.0 {
                100.0
            } else {
                avg_gain / avg_loss
            };
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            self.rsi_values.push(rsi);
        }
    }

    pub fn get_rsi_values(&self) -> Vec<f64> {
        self.rsi_values.clone()
    }
}
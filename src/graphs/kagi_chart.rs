// src/graphs/kagi_chart
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KagiChart {
    prices: Vec<f64>,
    reversal_amount: f64,
    lines: Vec<f64>,
}

#[wasm_bindgen]
impl KagiChart {
    pub fn new(prices: Vec<f64>, reversal_amount: f64) -> Self {
        KagiChart {
            prices,
            reversal_amount,
            lines: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        if self.prices.is_empty() {
            return;
        }

        let mut direction = 1; // 1 for up, -1 for down
        let mut last_price = self.prices[0];
        self.lines.push(last_price);

        for &price in &self.prices[1..] {
            let price_diff = price - last_price;

            if (direction == 1 && price_diff >= 0.0) || (direction == -1 && price_diff <= 0.0) {
                // Continue in the same direction
                self.lines.push(price);
                last_price = price;
            } else if (direction == 1 && price_diff <= -self.reversal_amount)
                || (direction == -1 && price_diff >= self.reversal_amount)
            {
                // Reverse direction
                direction *= -1;
                self.lines.push(price);
                last_price = price;
            }
        }
    }

    pub fn get_lines(&self) -> Vec<f64> {
        self.lines.clone()
    }
}
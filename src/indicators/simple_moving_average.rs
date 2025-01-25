use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SimpleMovingAverage {
    period: usize,
    prices: Vec<f64>,
    sma_values: Vec<f64>,
}

#[wasm_bindgen]
impl SimpleMovingAverage {
    pub fn new(period: usize, prices: Vec<f64>) -> Self {
        SimpleMovingAverage {
            period,
            prices,
            sma_values: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.prices.len();
        if len < self.period {
            return; // Not enough data to calculate SMA
        }

        for i in (self.period - 1)..len {
            let sum: f64 = self.prices[(i - self.period + 1)..=i].iter().sum();
            let sma = sum / self.period as f64;
            self.sma_values.push(sma);
        }
    }

    pub fn get_sma_values(&self) -> Vec<f64> {
        self.sma_values.clone()
    }
}
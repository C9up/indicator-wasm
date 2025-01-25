use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SimpleMovingAverage {
    prices: Vec<f64>,
}

#[wasm_bindgen]
impl SimpleMovingAverage {

    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        SimpleMovingAverage {
            prices
        }
    }
    pub fn period(&mut self, period: usize) -> Vec<f64> {
        let len = self.prices.len();
        if len < period {
            return vec![];
        }

        let mut sma = Vec::new();
        for i in 0..len {
            if i >= period - 1 {
                let sum: f64 = self.prices[i + 1 - period..=i].iter().sum();
                sma.push(sum / period as f64);
            } else {
                sma.push(f64::NAN);
            }
        }
        sma
    }
}
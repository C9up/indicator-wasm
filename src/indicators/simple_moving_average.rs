use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SimpleMovingAverage {
    prices: Vec<f64>,
}

#[wasm_bindgen]
impl SimpleMovingAverage {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        SimpleMovingAverage { prices }
    }

    pub fn period(&self, period: usize) -> Vec<f64> {
        let len = self.prices.len();
        if len < period {
            return vec![];
        }

        let mut sma = Vec::with_capacity(len);
        let mut window_sum: f64 = self.prices[..period].iter().sum(); // Somme initiale

        for i in 0..len {
            if i >= period {
                // Mise à jour de la somme glissante
                window_sum += self.prices[i] - self.prices[i - period];
            }

            if i >= period - 1 {
                sma.push(window_sum / period as f64);
            } else {
                sma.push(f64::NAN);
            }
        }

        sma
    }
}
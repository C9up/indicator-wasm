use wasm_bindgen::prelude::*;
use crate::utils::calculate_sma;

#[wasm_bindgen]
pub struct BollingerBands {
    prices: Vec<f64>
}

#[wasm_bindgen]
pub struct BollingerBandsResult {
    middle_band: Vec<f64>,
    upper_band: Vec<f64>,
    lower_band: Vec<f64>,
}

#[wasm_bindgen]
impl BollingerBands {
    pub fn new(prices: Vec<f64>) -> Self {
        BollingerBands {
            prices
        }
    }

    fn calculate_std_dev(&self, data: &[f64], sma: &[f64], period: usize) -> Vec<f64> {
        let mut std_dev = Vec::new();
        for i in (period - 1)..data.len() {
            let mean = sma[i - (period - 1)];
            let variance: f64 = data[(i - period + 1)..=i]
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / period as f64;
            std_dev.push(variance.sqrt());
        }
        std_dev
    }

    pub fn calculate(&mut self, period: usize, multiplier: f64) {
        let mut middle_band = Vec::new();
        let mut upper_band = Vec::new();
        let mut lower_band = Vec::new();

        let len = self.prices.len();
        if len < period {
            return; // Not enough data to calculate Bollinger Bands
        }

        // Calculate the Middle Band (SMA)
        middle_band = calculate_sma(&self.prices, period);

        // Calculate the Standard Deviation
        let std_dev = self.calculate_std_dev(&self.prices, &middle_band, period);

        // Calculate the Upper and Lower Bands
        for i in 0..middle_band.len() {
            let upper = middle_band[i] + (std_dev[i] * multiplier);
            let lower = middle_band[i] - (std_dev[i] * multiplier);
            upper_band.push(upper);
            lower_band.push(lower);
        }

        BollingerBandsResult {
            middle_band,
            upper_band,
            lower_band,
        };
    }
}
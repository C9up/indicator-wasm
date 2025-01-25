use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BollingerBands {
    period: usize,
    multiplier: f64,
    prices: Vec<f64>,
    middle_band: Vec<f64>,
    upper_band: Vec<f64>,
    lower_band: Vec<f64>,
}

#[wasm_bindgen]
impl BollingerBands {
    pub fn new(period: usize, multiplier: f64, prices: Vec<f64>) -> Self {
        BollingerBands {
            period,
            multiplier,
            prices,
            middle_band: Vec::new(),
            upper_band: Vec::new(),
            lower_band: Vec::new(),
        }
    }

    fn calculate_sma(&self, data: &[f64], period: usize) -> Vec<f64> {
        let mut sma = Vec::new();
        for i in (period - 1)..data.len() {
            let sum: f64 = data[(i - period + 1)..=i].iter().sum();
            sma.push(sum / period as f64);
        }
        sma
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

    pub fn calculate(&mut self) {
        let len = self.prices.len();
        if len < self.period {
            return; // Not enough data to calculate Bollinger Bands
        }

        // Calculate the Middle Band (SMA)
        self.middle_band = self.calculate_sma(&self.prices, self.period);

        // Calculate the Standard Deviation
        let std_dev = self.calculate_std_dev(&self.prices, &self.middle_band, self.period);

        // Calculate the Upper and Lower Bands
        for i in 0..self.middle_band.len() {
            let upper = self.middle_band[i] + (std_dev[i] * self.multiplier);
            let lower = self.middle_band[i] - (std_dev[i] * self.multiplier);
            self.upper_band.push(upper);
            self.lower_band.push(lower);
        }
    }

    pub fn get_middle_band(&self) -> Vec<f64> {
        self.middle_band.clone()
    }

    pub fn get_upper_band(&self) -> Vec<f64> {
        self.upper_band.clone()
    }

    pub fn get_lower_band(&self) -> Vec<f64> {
        self.lower_band.clone()
    }
}
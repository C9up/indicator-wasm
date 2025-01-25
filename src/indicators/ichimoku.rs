// src/indicator/ichimoku.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ichimoku {
    high_prices: Vec<f64>,
    low_prices: Vec<f64>,
    close_prices: Vec<f64>,
    tenkan_sen: Vec<f64>,
    kijun_sen: Vec<f64>,
    senkou_span_a: Vec<f64>,
    senkou_span_b: Vec<f64>,
    chikou_span: Vec<f64>,
}

#[wasm_bindgen]
impl Ichimoku {
    pub fn new(high_prices: Vec<f64>, low_prices: Vec<f64>, close_prices: Vec<f64>) -> Self {
        Ichimoku {
            high_prices,
            low_prices,
            close_prices,
            tenkan_sen: Vec::new(),
            kijun_sen: Vec::new(),
            senkou_span_a: Vec::new(),
            senkou_span_b: Vec::new(),
            chikou_span: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.high_prices.len();
        if len < 52 {
            return; // Not enough data to calculate all components
        }

        // Calculate Tenkan-sen (Conversion Line)
        for i in 8..len {
            let highest_high = self.high_prices[i - 8..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[i - 8..=i].iter().cloned().fold(f64::MAX, f64::min);
            self.tenkan_sen.push((highest_high + lowest_low) / 2.0);
        }

        // Calculate Kijun-sen (Base Line)
        for i in 25..len {
            let highest_high = self.high_prices[i - 25..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[i - 25..=i].iter().cloned().fold(f64::MAX, f64::min);
            self.kijun_sen.push((highest_high + lowest_low) / 2.0);
        }

        // Calculate Senkou Span A (Leading Span A)
        for i in 0..self.tenkan_sen.len() {
            if i < 26 {
                self.senkou_span_a.push(f64::NAN); // Not enough data to calculate
            } else {
                self.senkou_span_a.push((self.tenkan_sen[i] + self.kijun_sen[i]) / 2.0);
            }
        }

        // Calculate Senkou Span B (Leading Span B)
        for i in 51..len {
            let highest_high = self.high_prices[i - 51..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[i - 51..=i].iter().cloned().fold(f64::MAX, f64::min);
            self.senkou_span_b.push((highest_high + lowest_low) / 2.0);
        }

        // Calculate Chikou Span (Lagging Span)
        for i in 0..len {
            if i < 26 {
                self.chikou_span.push(f64::NAN); // Not enough data to calculate
            } else {
                self.chikou_span.push(self.close_prices[i - 26]);
            }
        }
    }

    pub fn get_tenkan_sen(&self) -> Vec<f64> {
        self.tenkan_sen.clone()
    }

    pub fn get_kijun_sen(&self) -> Vec<f64> {
        self.kijun_sen.clone()
    }

    pub fn get_senkou_span_a(&self) -> Vec<f64> {
        self.senkou_span_a.clone()
    }

    pub fn get_senkou_span_b(&self) -> Vec<f64> {
        self.senkou_span_b.clone()
    }

    pub fn get_chikou_span(&self) -> Vec<f64> {
        self.chikou_span.clone()
    }
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ichimoku {
    high_prices: Vec<f64>,
    low_prices: Vec<f64>,
    close_prices: Vec<f64>,
}

#[wasm_bindgen]
pub struct IchimokuResult {
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
            close_prices
        }
    }

    pub fn calculate(&mut self) -> IchimokuResult {
        let len = self.high_prices.len();
        let mut tenkan_sen = Vec::new();
        let mut kijun_sen = Vec::new();
        let mut senkou_span_a = Vec::new();
        let mut senkou_span_b = Vec::new();
        let mut chikou_span = Vec::new();

        if len < 52 {
            return IchimokuResult {
                tenkan_sen: vec![],
                kijun_sen: vec![],
                senkou_span_a: vec![],
                senkou_span_b: vec![],
                chikou_span: vec![],
            }; // Not enough data to calculate all components
        }

        // Calculate Tenkan-sen (Conversion Line)
        for i in 8..len {
            let highest_high = self.high_prices[i - 8..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[i - 8..=i].iter().cloned().fold(f64::MAX, f64::min);
            tenkan_sen.push((highest_high + lowest_low) / 2.0);
        }

        // Calculate Kijun-sen (Base Line)
        for i in 25..len {
            let highest_high = self.high_prices[i - 25..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[i - 25..=i].iter().cloned().fold(f64::MAX, f64::min);
            kijun_sen.push((highest_high + lowest_low) / 2.0);
        }

        // Calculate Senkou Span A (Leading Span A)
        for i in 0..tenkan_sen.len() {
            if i < 26 {
                senkou_span_a.push(f64::NAN); // Not enough data to calculate
            } else {
                senkou_span_a.push((tenkan_sen[i] + kijun_sen[i]) / 2.0);
            }
        }

        // Calculate Senkou Span B (Leading Span B)
        for i in 51..len {
            let highest_high = self.high_prices[i - 51..=i].iter().cloned().fold(f64::MIN, f64::max);
            let lowest_low = self.low_prices[i - 51..=i].iter().cloned().fold(f64::MAX, f64::min);
            senkou_span_b.push((highest_high + lowest_low) / 2.0);
        }

        // Calculate Chikou Span (Lagging Span)
        for i in 0..len {
            if i < 26 {
                chikou_span.push(f64::NAN); // Not enough data to calculate
            } else {
                chikou_span.push(self.close_prices[i - 26]);
            }
        }

        IchimokuResult {
            tenkan_sen,
            kijun_sen,
            senkou_span_a,
            senkou_span_b,
            chikou_span,
        }

    }
}
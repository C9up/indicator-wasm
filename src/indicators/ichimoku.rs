use wasm_bindgen::prelude::*;
use crate::highest_lowest::calculate_high_low;
use crate::low_high_open_close_volume_date_to_array::{low_high_open_close_volume_date_deserialize, low_high_open_close_volume_date_to_array};

#[wasm_bindgen]
pub struct IchimokuCloud {
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
impl IchimokuResult {
    #[wasm_bindgen(getter)]
    pub fn tenkan_sen(&self) -> Vec<f64> {
        self.tenkan_sen.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kijun_sen(&self) -> Vec<f64> {
        self.kijun_sen.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn senkou_span_a(&self) -> Vec<f64> {
        self.senkou_span_a.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn senkou_span_b(&self) -> Vec<f64> {
        self.senkou_span_b.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn chikou_span(&self) -> Vec<f64> {
        self.chikou_span.clone()
    }
}

#[wasm_bindgen]
impl IchimokuCloud {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: JsValue) -> Self {
        // Convert the input prices into high, low, and close arrays
        let segment = low_high_open_close_volume_date_deserialize(low_high_open_close_volume_date_to_array(prices)
            .expect("Failed to convert market data"));
        IchimokuCloud {
            high_prices: segment.highs,
            low_prices: segment.lows,
            close_prices: segment.closes,
        }
    }

    // Utility method to calculate the average of two values
    fn calculate_average(&self, a: f64, b: f64) -> f64 {
        (a + b) / 2.0
    }

    pub fn calculate(&self) -> IchimokuResult {
        let len = self.high_prices.len();
        // Return empty results if there isn't enough data
        if len < 52 {
            return IchimokuResult {
                tenkan_sen: Vec::new(),
                kijun_sen: Vec::new(),
                senkou_span_a: Vec::new(),
                senkou_span_b: Vec::new(),
                chikou_span: Vec::new(),
            };
        }

        // Pre-allocate vectors with the required capacity
        let mut tenkan_sen = Vec::with_capacity(len - 8);
        let mut kijun_sen = Vec::with_capacity(len - 25);
        let mut senkou_span_a = Vec::with_capacity(len - 26);
        let mut senkou_span_b = Vec::with_capacity(len - 51);
        let mut chikou_span = Vec::with_capacity(len);

        // Single loop to calculate tenkan_sen, kijun_sen, and senkou_span_b
        for i in 51..len {
            // Calculate Tenkan-sen (Conversion Line) if i >= 8
            if i >= 8 {
                let (highest_high, lowest_low) = calculate_high_low(&self.high_prices, &self.low_prices, i - 8, i);
                tenkan_sen.push(self.calculate_average(highest_high, lowest_low));
            }

            // Calculate Kijun-sen (Base Line) if i >= 25
            if i >= 25 {
                let (highest_high, lowest_low) = calculate_high_low(&self.high_prices, &self.low_prices, i - 25, i);
                kijun_sen.push(self.calculate_average(highest_high, lowest_low));
            }

            // Calculate Senkou Span B (Leading Span B) for all i >= 51
            let (highest_high, lowest_low) = calculate_high_low(&self.high_prices, &self.low_prices, i - 51, i);
            senkou_span_b.push(self.calculate_average(highest_high, lowest_low));
        }

        // Calculate Senkou Span A (Leading Span A)
        for i in 0..tenkan_sen.len() {
            if i < 26 {
                // Not enough data to calculate Senkou Span A
                senkou_span_a.push(f64::NAN);
            } else {
                senkou_span_a.push(self.calculate_average(tenkan_sen[i], kijun_sen[i]));
            }
        }

        // Calculate Chikou Span (Lagging Span)
        for i in 0..len {
            if i < 26 {
                // Not enough data to calculate Chikou Span
                chikou_span.push(f64::NAN);
            } else {
                chikou_span.push(self.close_prices[i - 26]);
            }
        }

        // Return the calculated Ichimoku Cloud components
        IchimokuResult {
            tenkan_sen,
            kijun_sen,
            senkou_span_a,
            senkou_span_b,
            chikou_span,
        }
    }
}
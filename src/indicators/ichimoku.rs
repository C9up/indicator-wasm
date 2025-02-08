use wasm_bindgen::prelude::*;
use crate::highest_lowest::calculate_high_low;
use crate::low_high_open_close_volume_date_to_array::{
    low_high_open_close_volume_date_deserialize, low_high_open_close_volume_date_to_array,
};

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
        // Convert input data into segments (highs, lows, closes)
        let segment = low_high_open_close_volume_date_deserialize(
            low_high_open_close_volume_date_to_array(prices)
                .expect("Failed to convert market data"),
        );
        IchimokuCloud {
            high_prices: segment.highs,
            low_prices: segment.lows,
            close_prices: segment.closes,
        }
    }

    /// Utility method to calculate the average of two values
    fn calculate_average(&self, a: f64, b: f64) -> f64 {
        (a + b) / 2.0
    }

    /// Calculates the Ichimoku Cloud components using:
    /// - `tenkan_period` for Tenkan-sen (conversion line)
    /// - `kijun_period` for Kijun-sen (base line)
    /// - `senkou_span_b_period` for Senkou Span B (delayed span)
    ///
    /// Chikou Span is calculated with a fixed 26-period shift.
    pub fn calculate(
        &self,
        tenkan_period: usize,
        kijun_period: usize,
        senkou_span_b_period: usize,
    ) -> IchimokuResult {
        let len = self.high_prices.len();

        // Ensure there is enough data for calculations.
        // Here, we require at least enough data for Senkou Span B and the Chikou Span shift.
        if len < senkou_span_b_period || len < tenkan_period || len < kijun_period || len < 26 {
            return IchimokuResult {
                tenkan_sen: Vec::new(),
                kijun_sen: Vec::new(),
                senkou_span_a: Vec::new(),
                senkou_span_b: Vec::new(),
                chikou_span: Vec::new(),
            };
        }

        // Start calculation at index (senkou_span_b_period - 1),
        // which is the first index where we can calculate Senkou Span B.
        let start_index = senkou_span_b_period - 1;
        let mut tenkan_sen = Vec::with_capacity(len - start_index);
        let mut kijun_sen = Vec::with_capacity(len - start_index);
        let mut senkou_span_b = Vec::with_capacity(len - start_index);

        // Single loop calculation for Tenkan-sen, Kijun-sen, and Senkou Span B
        for i in start_index..len {
            // Calculate Tenkan-sen if enough data is available
            if i >= tenkan_period - 1 {
                let window_start = i + 1 - tenkan_period;
                let (highest, lowest) =
                    calculate_high_low(&self.high_prices, &self.low_prices, window_start, i);
                tenkan_sen.push(self.calculate_average(highest, lowest));
            } else {
                tenkan_sen.push(f64::NAN);
            }

            // Calculate Kijun-sen if enough data is available
            if i >= kijun_period - 1 {
                let window_start = i + 1 - kijun_period;
                let (highest, lowest) =
                    calculate_high_low(&self.high_prices, &self.low_prices, window_start, i);
                kijun_sen.push(self.calculate_average(highest, lowest));
            } else {
                kijun_sen.push(f64::NAN);
            }

            // Calculate Senkou Span B (always calculable in this loop)
            let window_start = i + 1 - senkou_span_b_period;
            let (highest, lowest) =
                calculate_high_low(&self.high_prices, &self.low_prices, window_start, i);
            senkou_span_b.push(self.calculate_average(highest, lowest));
        }

        // Calculate Senkou Span A: it is the average of Tenkan-sen and Kijun-sen,
        // shifted forward by 26 periods.
        let mut senkou_span_a = Vec::with_capacity(tenkan_sen.len());
        for i in 0..tenkan_sen.len() {
            if i < 26 {
                senkou_span_a.push(f64::NAN);
            } else {
                senkou_span_a.push(self.calculate_average(tenkan_sen[i], kijun_sen[i]));
            }
        }

        // Calculate Chikou Span (lagging line): shift the closing price by 26 periods.
        let mut chikou_span = Vec::with_capacity(len);
        for i in 0..len {
            if i < 26 {
                chikou_span.push(f64::NAN);
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

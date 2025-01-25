// src/indicator/macd.rs
use wasm_bindgen::prelude::*;
use crate::utils;

#[wasm_bindgen]
pub struct MovingAverageConvergenceDivergence {
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    prices: Vec<f64>,
    macd_line: Vec<f64>,
    signal_line: Vec<f64>,
    histogram: Vec<f64>,
}

#[wasm_bindgen]
impl MovingAverageConvergenceDivergence {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize, prices: Vec<f64>) -> Self {
        MovingAverageConvergenceDivergence {
            fast_period,
            slow_period,
            signal_period,
            prices,
            macd_line: Vec::new(),
            signal_line: Vec::new(),
            histogram: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.prices.len();
        if len < self.fast_period || len < self.slow_period || len < self.signal_period {
            return; // Not enough data to calculate MACD
        }

        // Calculate the fast and slow EMAs
        let fast_ema = utils::calculate_ema(&self.prices, self.fast_period);
        let slow_ema = utils::calculate_ema(&self.prices, self.slow_period);

        // Calculate the MACD Line
        for i in 0..fast_ema.len() {
            self.macd_line.push(fast_ema[i] - slow_ema[i]);
        }

        // Calculate the Signal Line (EMA of the MACD Line)
        self.signal_line = utils::calculate_ema(&self.macd_line, self.signal_period);

        // Calculate the Histogram
        for i in 0..self.macd_line.len() {
            self.histogram.push(self.macd_line[i] - self.signal_line[i]);
        }
    }

    pub fn get_macd_line(&self) -> Vec<f64> {
        self.macd_line.clone()
    }

    pub fn get_signal_line(&self) -> Vec<f64> {
        self.signal_line.clone()
    }

    pub fn get_histogram(&self) -> Vec<f64> {
        self.histogram.clone()
    }
}
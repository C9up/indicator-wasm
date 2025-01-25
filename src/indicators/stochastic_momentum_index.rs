// src/indicator/smi.rs
use wasm_bindgen::prelude::*;
use crate::utils;

#[wasm_bindgen]
pub struct StochasticMomentumIndex {
    period: usize,
    smoothing_period: usize,
    high_prices: Vec<f64>,
    low_prices: Vec<f64>,
    close_prices: Vec<f64>,
    smi_values: Vec<f64>,
}

#[wasm_bindgen]
impl StochasticMomentumIndex {
    pub fn new(period: usize, smoothing_period: usize, high_prices: Vec<f64>, low_prices: Vec<f64>, close_prices: Vec<f64>) -> Self {
        StochasticMomentumIndex {
            period,
            smoothing_period,
            high_prices,
            low_prices,
            close_prices,
            smi_values: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.high_prices.len();
        if len < self.period || len < self.smoothing_period {
            return; // Not enough data to calculate SMI
        }

        // Calculate midpoints and differences
        let mut midpoints = Vec::new();
        let mut diffs = Vec::new();
        for i in 0..len {
            let midpoint = (self.high_prices[i] + self.low_prices[i]) / 2.0;
            midpoints.push(midpoint);
            diffs.push(self.close_prices[i] - midpoint);
        }

        // Smooth the diffs and high-low ranges
        let smoothed_diffs = utils::calculate_ema(&diffs, self.smoothing_period);
        let smoothed_ranges = utils::calculate_ema(
            &self.high_prices.iter().zip(&self.low_prices).map(|(h, l)| h - l).collect::<Vec<f64>>(),
            self.smoothing_period,
        );

        // Calculate SMI
        for i in 0..smoothed_diffs.len() {
            let smi = (smoothed_diffs[i] / (smoothed_ranges[i] / 2.0)) * 100.0;
            self.smi_values.push(smi);
        }
    }

    pub fn get_smi_values(&self) -> Vec<f64> {
        self.smi_values.clone()
    }
}
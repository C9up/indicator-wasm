// src/indicator/smi.rs
use wasm_bindgen::prelude::*;
use crate::utils;

#[wasm_bindgen]
pub struct StochasticMomentumIndex {
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
}

#[wasm_bindgen]
impl StochasticMomentumIndex {

    #[wasm_bindgen(constructor)]
    pub fn new(highs: Vec<f64>, lows: Vec<f64>, closes: Vec<f64>) -> Self {
        StochasticMomentumIndex {
            highs,
            lows,
            closes
        }
    }

    pub fn calculate(&mut self, period: usize, smoothing_period: usize) -> Vec<f64> {
        let mut smi_values = Vec::new();
        let len = self.highs.len();
        if len < period || len < smoothing_period {
            return vec![]; // Not enough data to calculate SMI
        }

        // Calculate midpoints and differences
        let mut midpoints = Vec::new();
        let mut diffs = Vec::new();
        for i in 0..len {
            let midpoint = (self.highs[i] + self.lows[i]) / 2.0;
            midpoints.push(midpoint);
            diffs.push(self.closes[i] - midpoint);
        }

        // Smooth the diffs and high-low ranges
        let smoothed_diffs = utils::calculate_ema(&diffs, smoothing_period);
        let smoothed_ranges = utils::calculate_ema(
            &self.highs.iter().zip(&self.lows).map(|(h, l)| h - l).collect::<Vec<f64>>(),
            smoothing_period,
        );

        // Calculate SMI
        for i in 0..smoothed_diffs.len() {
            let smi = (smoothed_diffs[i] / (smoothed_ranges[i] / 2.0)) * 100.0;
            smi_values.push(smi);
        }

        smi_values
    }
}
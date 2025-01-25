// src/indicator/trends_meter.rs
use wasm_bindgen::prelude::*;
use crate::simple_moving_average::SimpleMovingAverage;
use crate::relative_strength_index::RelativeStrengthIndex;
use crate::moving_average_convergence_divergence::MovingAverageConvergenceDivergence;

#[wasm_bindgen]
pub struct TrendsMeter {
    sma: SimpleMovingAverage,
    rsi: RelativeStrengthIndex,
    macd: MovingAverageConvergenceDivergence,
    trend_scores: Vec<f64>,
}

#[wasm_bindgen]
impl TrendsMeter {
    pub fn new(
        sma_period: usize,
        rsi_period: usize,
        macd_fast_period: usize,
        macd_slow_period: usize,
        macd_signal_period: usize,
        prices: Vec<f64>,
    ) -> Self {
        let sma = SimpleMovingAverage::new(sma_period, prices.clone());
        let rsi = RelativeStrengthIndex::new(rsi_period, prices.clone());
        let macd = MovingAverageConvergenceDivergence::new(macd_fast_period, macd_slow_period, macd_signal_period, prices);

        TrendsMeter {
            sma,
            rsi,
            macd,
            trend_scores: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        // Calculate individual indicators
        self.sma.calculate();
        self.rsi.calculate();
        self.macd.calculate();

        let sma_values = self.sma.get_sma_values();
        let rsi_values = self.rsi.get_rsi_values();
        let macd_values = self.macd.get_macd_line();
        let macd_signal_values = self.macd.get_signal_line();

        // Combine indicators into a trend score
        for i in 0..sma_values.len() {
            let sma_score = if i > 0 {
                (sma_values[i] - sma_values[i - 1]).signum() * 50.0
            } else {
                0.0
            };

            let rsi_score = (rsi_values[i] - 50.0) / 50.0 * 100.0;
            let macd_score = (macd_values[i] - macd_signal_values[i]).signum() * 50.0;

            let trend_score = (sma_score + rsi_score + macd_score) / 3.0;
            self.trend_scores.push(trend_score);
        }
    }

    pub fn get_trend_scores(&self) -> Vec<f64> {
        self.trend_scores.clone()
    }
}
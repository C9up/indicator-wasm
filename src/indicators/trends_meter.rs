use wasm_bindgen::prelude::*;
use crate::simple_moving_average::SimpleMovingAverage;
use crate::relative_strength_index::RelativeStrengthIndex;
use crate::moving_average_convergence_divergence::MovingAverageConvergenceDivergence;

#[wasm_bindgen]
pub struct TrendsMeter {
    sma: SimpleMovingAverage,
    rsi: RelativeStrengthIndex,
    macd: MovingAverageConvergenceDivergence
}

#[wasm_bindgen]
impl TrendsMeter {

    #[wasm_bindgen(constructor)]
    pub fn new(
        prices: Vec<f64>,
    ) -> Self {
        let sma = SimpleMovingAverage::new(prices.clone());
        let rsi = RelativeStrengthIndex::new(prices.clone());
        let macd = MovingAverageConvergenceDivergence::new(prices.clone());

        TrendsMeter {
            sma,
            rsi,
            macd
        }
    }

    pub fn calculate(&mut self, sma_period: usize, rsi_period: usize, macd_fast_period: usize, macd_slow_period: usize, macd_signal_period: usize) -> Vec<f64> {
        // Calculate individual indicators
        let sma_values = self.sma.period(sma_period);
        let rsi_values = self.rsi.period(rsi_period);
        let macd = self.macd.period(macd_fast_period, macd_slow_period, macd_signal_period);

        let mut trend_scores = Vec::new();

        // Combine indicators into a trend score
        for i in 0..sma_values.len() {
            let sma_score = if i > 0 {
                (sma_values[i] - sma_values[i - 1]).signum() * 50.0
            } else {
                0.0
            };

            let rsi_score = (rsi_values[i] - 50.0) / 50.0 * 100.0;
            let macd_score = (macd.macd_line[i] - macd.signal_line[i]).signum() * 50.0;

            let trend_score = (sma_score + rsi_score + macd_score) / 3.0;
            trend_scores.push(trend_score);
        }

        trend_scores
    }
}
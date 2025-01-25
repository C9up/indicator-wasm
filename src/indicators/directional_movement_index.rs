// src/indicator/dmi.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DirectionalMovementIndex {
    period: usize,
    high_prices: Vec<f64>,
    low_prices: Vec<f64>,
    close_prices: Vec<f64>,
    plus_di: Vec<f64>,
    minus_di: Vec<f64>,
    adx: Vec<f64>,
}

#[wasm_bindgen]
impl DirectionalMovementIndex {
    pub fn new(period: usize, high_prices: Vec<f64>, low_prices: Vec<f64>, close_prices: Vec<f64>) -> Self {
        DirectionalMovementIndex {
            period,
            high_prices,
            low_prices,
            close_prices,
            plus_di: Vec::new(),
            minus_di: Vec::new(),
            adx: Vec::new(),
        }
    }

    pub fn calculate(&mut self) {
        let len = self.high_prices.len();
        if len < self.period {
            return;
        }

        let mut tr = vec![0.0; len]; // True Range
        let mut plus_dm = vec![0.0; len]; // +DM
        let mut minus_dm = vec![0.0; len]; // -DM

        // Calculate True Range, +DM, and -DM
        for i in 1..len {
            let high = self.high_prices[i];
            let low = self.low_prices[i];
            let prev_close = self.close_prices[i - 1];

            tr[i] = (high - low).max((high - prev_close).abs()).max((low - prev_close).abs());

            let up_move = high - self.high_prices[i - 1];
            let down_move = self.low_prices[i - 1] - low;

            if up_move > down_move && up_move > 0.0 {
                plus_dm[i] = up_move;
            }
            if down_move > up_move && down_move > 0.0 {
                minus_dm[i] = down_move;
            }
        }

        // Calculate +DI and -DI
        let mut plus_di = vec![0.0; len];
        let mut minus_di = vec![0.0; len];
        let mut tr_sum: f64 = tr[1..=self.period].iter().sum();
        let mut plus_dm_sum: f64 = plus_dm[1..=self.period].iter().sum();
        let mut minus_dm_sum: f64 = minus_dm[1..=self.period].iter().sum();

        plus_di[self.period] = 100.0 * (plus_dm_sum / tr_sum);
        minus_di[self.period] = 100.0 * (minus_dm_sum / tr_sum);

        for i in (self.period + 1)..len {
            tr_sum = tr_sum - tr[i - self.period] + tr[i];
            plus_dm_sum = plus_dm_sum - plus_dm[i - self.period] + plus_dm[i];
            minus_dm_sum = minus_dm_sum - minus_dm[i - self.period] + minus_dm[i];

            plus_di[i] = 100.0 * (plus_dm_sum / tr_sum);
            minus_di[i] = 100.0 * (minus_dm_sum / tr_sum);
        }

        // Calculate ADX
        let mut dx = vec![0.0; len]; // Directional Movement Index
        for i in self.period..len {
            dx[i] = 100.0 * ((plus_di[i] - minus_di[i]).abs() / (plus_di[i] + minus_di[i]));
        }

        let mut adx = vec![0.0; len];
        let dx_sum: f64 = dx[self.period..=2 * self.period - 1].iter().sum();
        adx[2 * self.period - 1] = dx_sum / self.period as f64;

        for i in 2 * self.period..len {
            adx[i] = (adx[i - 1] * (self.period as f64 - 1.0) + dx[i]) / self.period as f64;
        }

        // Store results
        self.plus_di = plus_di;
        self.minus_di = minus_di;
        self.adx = adx;
    }

    pub fn get_plus_di(&self) -> Vec<f64> {
        self.plus_di.clone()
    }

    pub fn get_minus_di(&self) -> Vec<f64> {
        self.minus_di.clone()
    }

    pub fn get_adx(&self) -> Vec<f64> {
        self.adx.clone()
    }
}
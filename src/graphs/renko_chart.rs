use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RenkoChart {
    prices: Vec<f64>,
}

#[wasm_bindgen]
impl RenkoChart {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        RenkoChart { prices }
    }

    pub fn calculate(&mut self, brick_size: f64) -> Vec<f64> {
        if self.prices.is_empty() {
            return vec![]; // No data to process
        }

        let mut bricks = Vec::new();
        let mut last_brick = self.prices[0];
        bricks.push(last_brick);

        let mut trend: i32 = 0; // 1 for uptrend, -1 for downtrend

        for &price in &self.prices[1..] {
            let price_diff = price - last_brick;

            if (trend >= 0 && price_diff >= brick_size) || (trend < 0 && price_diff <= -brick_size) {
                self.add_bricks(&mut bricks, &mut last_brick, price_diff, brick_size, trend);
                trend = if price_diff >= brick_size { 1 } else { -1 };
            } else if (trend >= 0 && price_diff <= -brick_size) || (trend < 0 && price_diff >= brick_size) {
                self.add_bricks(&mut bricks, &mut last_brick, price_diff, brick_size, -trend);
                trend = if price_diff >= brick_size { 1 } else { -1 };
            }
        }

        bricks
    }

    fn add_bricks(
        &self,
        bricks: &mut Vec<f64>,
        last_brick: &mut f64,
        price_diff: f64,
        brick_size: f64,
        trend: i32,
    ) {
        let num_bricks = (price_diff.abs() / brick_size).floor() as i32;
        let step = brick_size * trend as f64;

        for _ in 0..num_bricks {
            *last_brick += step;
            bricks.push(*last_brick);
        }
    }
}
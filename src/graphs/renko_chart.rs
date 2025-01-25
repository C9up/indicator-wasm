use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RenkoChart {
    brick_size: f64,
    prices: Vec<f64>,
    bricks: Vec<f64>,
    last_brick: f64,
    trend: i32, // 1 for uptrend, -1 for downtrend
}

#[wasm_bindgen]
impl RenkoChart {
    #[wasm_bindgen(constructor)]
    pub fn new(brick_size: f64, prices: Vec<f64>) -> Self {
        RenkoChart {
            brick_size,
            prices,
            bricks: Vec::new(),
            last_brick: 0.0,
            trend: 0, // Start with no trend
        }
    }

    pub fn calculate(&mut self) {
        if self.prices.is_empty() {
            return; // No data to process
        }

        // Initialize the first brick
        self.last_brick = self.prices[0];
        self.bricks.push(self.last_brick);

        for &price in &self.prices[1..] {
            let price_diff = price - self.last_brick;

            if self.trend >= 0 {
                // Uptrend
                if price_diff >= self.brick_size {
                    let num_bricks = (price_diff / self.brick_size).floor() as i32;
                    for _ in 0..num_bricks {
                        self.last_brick += self.brick_size;
                        self.bricks.push(self.last_brick);
                    }
                    self.trend = 1; // Confirm uptrend
                } else if price_diff <= -self.brick_size {
                    // Trend reversal to downtrend
                    let num_bricks = (-price_diff / self.brick_size).floor() as i32;
                    for _ in 0..num_bricks {
                        self.last_brick -= self.brick_size;
                        self.bricks.push(self.last_brick);
                    }
                    self.trend = -1; // Confirm downtrend
                }
            } else {
                // Downtrend
                if price_diff <= -self.brick_size {
                    let num_bricks = (-price_diff / self.brick_size).floor() as i32;
                    for _ in 0..num_bricks {
                        self.last_brick -= self.brick_size;
                        self.bricks.push(self.last_brick);
                    }
                    self.trend = -1; // Confirm downtrend
                } else if price_diff >= self.brick_size {
                    // Trend reversal to uptrend
                    let num_bricks = (price_diff / self.brick_size).floor() as i32;
                    for _ in 0..num_bricks {
                        self.last_brick += self.brick_size;
                        self.bricks.push(self.last_brick);
                    }
                    self.trend = 1; // Confirm uptrend
                }
            }
        }
    }

    pub fn get_bricks(&self) -> Vec<f64> {
        self.bricks.clone()
    }
}
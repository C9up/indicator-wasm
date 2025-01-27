use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Yang, // Bullish trend
    Yin,  // Bearish trend
}

#[wasm_bindgen]
pub struct KagiChart {
    prices: Vec<f64>,
}

#[wasm_bindgen]
pub struct KagiResult {
    prices: Vec<f64>,
    directions: Vec<String>,
}

#[wasm_bindgen]
impl KagiResult {
    #[wasm_bindgen(getter)]
    pub fn prices(&self) -> Vec<f64> {
        self.prices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn directions(&self) -> Vec<String> {
        self.directions.clone()
    }
}

#[wasm_bindgen]
impl KagiChart {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        KagiChart { prices }
    }

    fn update_direction(
        &self,
        price: f64,
        current_price: &mut f64,
        reversal_amount: f64,
        current_direction: &mut Direction,
        result_prices: &mut Vec<f64>,
        result_directions: &mut Vec<String>,
    ) {
        match current_direction {
            Direction::Yang => {
                if price >= *current_price {
                    // Bullish trend, continue rising
                    *current_price = price;
                } else if *current_price - price >= reversal_amount {
                    // Reverse to Yin (bearish)
                    result_prices.push(*current_price);
                    result_directions.push("Yang".to_string());
                    *current_direction = Direction::Yin;
                    *current_price = price;
                }
            }
            Direction::Yin => {
                if price <= *current_price {
                    // Bearish trend, continue falling
                    *current_price = price;
                } else if price - *current_price >= reversal_amount {
                    // Reverse to Yang (bullish)
                    result_prices.push(*current_price);
                    result_directions.push("Yin".to_string());
                    *current_direction = Direction::Yang;
                    *current_price = price;
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn calculate(&mut self, reversal_amount: f64) -> KagiResult {
        if reversal_amount <= 0.0 || self.prices.is_empty() {
            return KagiResult {
                prices: vec![],
                directions: vec![],
            };
        }

        let mut result_prices = Vec::new();
        let mut result_directions = Vec::new();

        let mut current_direction = Direction::Yang;
        let mut current_price = self.prices[0];

        for &price in &self.prices[1..] {
            self.update_direction(
                price,
                &mut current_price,
                reversal_amount,
                &mut current_direction,
                &mut result_prices,
                &mut result_directions,
            );
        }

        result_prices.push(current_price);
        result_directions.push(match current_direction {
            Direction::Yang => "Yang".to_string(),
            Direction::Yin => "Yin".to_string(),
        });

        KagiResult {
            prices: result_prices,
            directions: result_directions,
        }
    }
}
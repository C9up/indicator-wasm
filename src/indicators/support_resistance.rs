use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// Define a struct to represent support/resistance levels
#[wasm_bindgen]
pub struct SupportResistanceLevel {
    price: f64,
    occurrences: usize,
}

#[wasm_bindgen]
impl SupportResistanceLevel {
    #[wasm_bindgen(getter)]
    pub fn price(&self) -> f64 {
        self.price
    }

    #[wasm_bindgen(getter)]
    pub fn occurrences(&self) -> usize {
        self.occurrences
    }
}

#[wasm_bindgen]
pub struct SupportResistance {
    prices: Vec<f64>,
}

#[wasm_bindgen]
impl SupportResistance {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> Self {
        SupportResistance {
            prices
        }
    }

    /// Returns support and resistance levels that appear more than the given threshold
    #[wasm_bindgen]
    pub fn calculate(&self, threshold: usize) -> Vec<SupportResistanceLevel> {

        let mut levels: HashMap<i64, usize> = HashMap::with_capacity(1000);

        for &price in &self.prices {
            let rounded_price = (price * 100.0).round() as i64; // Convert to integer
            *levels.entry(rounded_price).or_insert(0) += 1;
        }

        levels
            .iter()
            .filter(|&(_, &count)| count >= threshold)
            .map(|(&price, &occurrences)| SupportResistanceLevel {
                price: price as f64 / 100.0, // Convert back to f64 with 2 decimals
                occurrences,
            })
            .collect()
    }
}
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

// Define a struct to represent support/resistance levels
#[derive(Serialize)]
struct SupportResistanceLevel {
    price: f64,
    occurrences: usize,
}

#[wasm_bindgen]
pub struct SupportResistance {
    prices: Vec<f64>,
    levels: HashMap<i64, usize>, // Change key type to i64
}

#[wasm_bindgen]
impl SupportResistance {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> SupportResistance {
        let mut sr = SupportResistance {
            prices,
            levels: HashMap::new(),
        };
        sr.calculate_levels();
        sr
    }

    fn calculate_levels(&mut self) {
        for &price in &self.prices {
            let rounded_price = (price * 100.0).round() as i64; // Convert to integer
            *self.levels.entry(rounded_price).or_insert(0) += 1;
        }
    }

    /// Returns support and resistance levels that appear more than the given threshold
    #[wasm_bindgen]
    pub fn get_levels(&self, threshold: usize) -> JsValue {
        let filtered_levels: Vec<SupportResistanceLevel> = self.levels.iter()
            .filter(|&(_, &count)| count >= threshold)
            .map(|(&price, &occurrences)| SupportResistanceLevel {
                price: price as f64 / 100.0, // Convert back to f64 with 2 decimals
                occurrences
            })
            .collect();

        // Convert Rust data structure to JsValue using serde-wasm-bindgen
        to_value(&filtered_levels).unwrap()
    }
}

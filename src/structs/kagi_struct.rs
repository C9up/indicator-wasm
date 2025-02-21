use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq, Serialize)]
pub enum Direction {
    Yang,
    Yin,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct KagiResult {
    #[wasm_bindgen(getter_with_clone)]
    pub prices: Vec<f64>,
    #[wasm_bindgen(getter_with_clone)]
    pub directions: Vec<String>,
}

#[wasm_bindgen]
impl KagiResult {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>, directions: Vec<String>) -> KagiResult {
        KagiResult { prices, directions }
    }
}

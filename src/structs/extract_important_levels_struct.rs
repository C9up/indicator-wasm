use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct DataEntry {
    pub pivot_point: f64,
    pub resistance1: f64,
    pub resistance2: f64,
    pub support1: f64,
    pub support2: f64,
}
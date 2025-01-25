use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PivotPoints {
    pivot: f64,
    resistance1: f64,
    resistance2: f64,
    support1: f64,
    support2: f64,
}

#[wasm_bindgen]
impl PivotPoints {
    pub fn new(high: f64, low: f64, close: f64) -> Self {
        // Calculate the pivot point and support/resistance levels
        let pivot = (high + low + close) / 3.0;
        let resistance1 = (2.0 * pivot) - low;
        let support1 = (2.0 * pivot) - high;
        let resistance2 = pivot + (high - low);
        let support2 = pivot - (high - low);

        PivotPoints {
            pivot,
            resistance1,
            resistance2,
            support1,
            support2,
        }
    }

    pub fn get_pivot(&self) -> f64 {
        self.pivot
    }

    pub fn get_resistance1(&self) -> f64 {
        self.resistance1
    }

    pub fn get_resistance2(&self) -> f64 {
        self.resistance2
    }

    pub fn get_support1(&self) -> f64 {
        self.support1
    }

    pub fn get_support2(&self) -> f64 {
        self.support2
    }
}
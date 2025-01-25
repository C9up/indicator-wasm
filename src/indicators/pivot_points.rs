use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PivotPoints {
    pub pivot: f64,
    pub resistance1: f64,
    pub resistance2: f64,
    pub support1: f64,
    pub support2: f64,
}

#[wasm_bindgen]
pub struct Pivot {
    high: f64,
    low: f64,
    close: f64,
}

#[wasm_bindgen]
impl Pivot {

    #[wasm_bindgen(constructor)]
    pub fn new(high: f64, low: f64, close: f64) -> Self {
        Pivot { high, low, close }
    }

    pub fn calculate(&self) -> PivotPoints {
        let pivot = (self.high + self.low + self.close) / 3.0;
        let resistance1 = (2.0 * pivot) - self.low;
        let support1 = (2.0 * pivot) - self.high;
        let resistance2 = pivot + (self.high - self.low);
        let support2 = pivot - (self.high - self.low);

        PivotPoints {
            pivot,
            resistance1,
            resistance2,
            support1,
            support2,
        }
    }
}

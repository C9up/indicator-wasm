use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Signal {
    signal_type: u8, // Privé (0 = entry, 1 = exit)
    price: f64,      // Privé
    index: usize,    // Privé
}

#[wasm_bindgen]
impl Signal {
    #[wasm_bindgen(constructor)]
    pub fn new(signal_type: u8, price: f64, index: usize) -> Signal {
        Signal {
            signal_type,
            price,
            index,
        }
    }

    // Getters pour accéder aux champs privés
    #[wasm_bindgen(getter)]
    pub fn signal_type(&self) -> u8 {
        self.signal_type
    }

    #[wasm_bindgen(getter)]
    pub fn price(&self) -> f64 {
        self.price
    }

    #[wasm_bindgen(getter)]
    pub fn index(&self) -> usize {
        self.index
    }
}
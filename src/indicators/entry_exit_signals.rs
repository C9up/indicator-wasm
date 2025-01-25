use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

// Define a struct to represent trade signals
#[derive(Serialize)]
struct Signal {
    signal_type: String, // "entry" or "exit"
    price: f64,
    index: usize,
}

#[wasm_bindgen]
pub struct EntryExitSignals {
    prices: Vec<f64>,
    signals: Vec<Signal>,
}

#[wasm_bindgen]
impl EntryExitSignals {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> EntryExitSignals {
        let mut instance = EntryExitSignals {
            prices,
            signals: Vec::new(),
        };
        instance.calculate_signals();
        instance
    }

    fn calculate_signals(&mut self) {
        if self.prices.len() < 2 {
            return; // Not enough data to calculate signals
        }

        let mut trend_up = false;
        for i in 1..self.prices.len() {
            let prev_price = self.prices[i - 1];
            let current_price = self.prices[i];

            if current_price > prev_price && !trend_up {
                // Entry signal (buy) when price starts increasing
                self.signals.push(Signal {
                    signal_type: "entry".to_string(),
                    price: current_price,
                    index: i,
                });
                trend_up = true;
            } else if current_price < prev_price && trend_up {
                // Exit signal (sell) when price starts decreasing
                self.signals.push(Signal {
                    signal_type: "exit".to_string(),
                    price: current_price,
                    index: i,
                });
                trend_up = false;
            }
        }
    }

    /// Get the entry and exit signals as JSON
    #[wasm_bindgen]
    pub fn get_signals(&self) -> JsValue {
        to_value(&self.signals).unwrap()
    }
}

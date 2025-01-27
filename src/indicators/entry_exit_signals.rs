use wasm_bindgen::prelude::*;

// Define a struct to represent trade signals
#[wasm_bindgen]
pub struct Signal {
    signal_type: String, // "entry" or "exit"
    price: f64,
    index: usize,
}

#[wasm_bindgen]
impl Signal {
    #[wasm_bindgen(getter)]
    pub fn signal_type(&self) -> String {
        self.signal_type.clone()
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

#[wasm_bindgen]
pub struct EntryExitSignals {
    prices: Vec<f64>
}

#[wasm_bindgen]
impl EntryExitSignals {

    #[wasm_bindgen(constructor)]
    pub fn new(prices: Vec<f64>) -> EntryExitSignals {
        EntryExitSignals {
            prices
        }
    }

    pub fn calculate(&mut self) -> Vec<Signal> {
        if self.prices.len() < 2 {
            // Log a warning or return an error in a real-world scenario
            return vec![];
        }

        let mut signals = Vec::new();
        let mut trend_up = false;

        for i in 1..self.prices.len() {
            let prev_price = self.prices[i - 1];
            let current_price = self.prices[i];

            if current_price > prev_price && !trend_up {
                // Entry signal (buy) when price starts increasing
                signals.push(Signal {
                    signal_type: "entry".to_string(),
                    price: current_price,
                    index: i,
                });
                trend_up = true;
            } else if current_price < prev_price && trend_up {
                // Exit signal (sell) when price starts decreasing
                signals.push(Signal {
                    signal_type: "exit".to_string(),
                    price: current_price,
                    index: i,
                });
                trend_up = false;
            }
        }

        signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_signals() {
        let prices = vec![100.0, 105.0, 102.0, 107.0, 106.0];
        let mut signals = EntryExitSignals::new(prices);
        let result = signals.calculate();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].signal_type(), "entry");
        assert_eq!(result[0].price(), 105.0);
        assert_eq!(result[1].signal_type(), "exit");
        assert_eq!(result[1].price(), 102.0);
    }

    #[test]
    fn test_not_enough_data() {
        let prices = vec![100.0];
        let mut signals = EntryExitSignals::new(prices);
        let result = signals.calculate();

        assert_eq!(result.len(), 0);
    }
}
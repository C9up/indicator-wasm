use crate::helpers::relative_strength_index_helper::calculate_gain_loss;
use wasm_bindgen::prelude::*;
use crate::jsvalue_to_f64;

// Function to calculate the Relative Strength Index (RSI)
#[wasm_bindgen]
pub fn relative_strength_index(data: JsValue, period: usize) -> Vec<f64> {
    let prices = jsvalue_to_f64(data);
    let mut rsi_values = Vec::new();

    for i in period..prices.len() {
        let (avg_gain, avg_loss) = calculate_gain_loss(&prices[i - period..i], period);

        // Avoid division by zero
        if avg_loss == 0.0 {
            rsi_values.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            rsi_values.push(rsi);
        }
    }

    rsi_values
}

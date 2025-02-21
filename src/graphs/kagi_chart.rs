use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::create_error;
use crate::kagi_struct::KagiResult;

#[wasm_bindgen]
pub fn kagi_chart(prices: Vec<f64>, reversal_amount: f64) -> Result<JsValue, JsValue> {
    // Validate input: reversal_amount must be greater than 0
    if reversal_amount <= 0.0 {
        return Err(create_error("Reversal amount must be greater than 0."));
    }
    // Validate input: prices vector must not be empty
    if prices.is_empty() {
        return Err(create_error("Prices vector must not be empty."));
    }

    let mut result_prices = Vec::new();
    let mut result_directions = Vec::new();

    let mut current_direction = true;  // true = Yang, false = Yin
    let mut current_price = prices[0];

    // Process the price data to calculate trend changes
    for &price in &prices[1..] {
        if current_direction {
            // Yang (bullish) mode: update the current price if the price increases,
            // or reverse to Yin if the drop exceeds the reversal amount.
            if price >= current_price {
                current_price = price;
            } else if current_price - price >= reversal_amount {
                result_prices.push(current_price);
                result_directions.push("Yang".to_string());
                current_direction = false; // Switch to Yin
                current_price = price;
            }
        } else {
            // Yin (bearish) mode: update the current price if the price decreases,
            // or reverse to Yang if the rise exceeds the reversal amount.
            if price <= current_price {
                current_price = price;
            } else if price - current_price >= reversal_amount {
                result_prices.push(current_price);
                result_directions.push("Yin".to_string());
                current_direction = true; // Switch to Yang
                current_price = price;
            }
        }
    }

    // Add the final price and its corresponding direction
    result_prices.push(current_price);
    result_directions.push(if current_direction { "Yang" } else { "Yin" }.to_string());

    let result = KagiResult {
        prices: result_prices,
        directions: result_directions,
    };

    // Convert the result to JsValue, handling any conversion errors
    let js_result = to_value(&result)
        .map_err(|e| create_error(&format!("Error converting result: {:?}", e)))?;

    Ok(js_result)
}

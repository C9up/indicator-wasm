use wasm_bindgen::prelude::*;
use crate::{create_error, serialize_to_js_value};

mod helpers {
    #[inline(always)]
    pub fn compute_brick_count(diff: f64, brick_size: f64) -> i32 {
        (diff.abs() / brick_size).floor() as i32
    }
}

#[wasm_bindgen]
pub fn renko_chart(prices: Vec<f64>, brick_size: f64) -> Result<JsValue, JsValue> {

    // Validate input: reversal_amount must be greater than 0
    if brick_size <= 0.0 {
        return Err(create_error("brick_size amount must be greater than 0."));
    }
    // Validate input: prices vector must not be empty
    if prices.is_empty() {
        return Err(create_error("Prices vector must not be empty."));
    }

    let mut result = Vec::with_capacity(prices.len() * 2);
    let mut last_price = prices[0];
    result.push(last_price);

    for price in prices.into_iter() {
        let diff = price - last_price;
        if diff.abs() >= brick_size {
            let brick_count = helpers::compute_brick_count(diff, brick_size);
            let direction = diff.signum();
            for _ in 0..brick_count {
                last_price += direction * brick_size;
                result.push(last_price);
            }
        }
    }

    Ok(serialize_to_js_value(&result)?)
}

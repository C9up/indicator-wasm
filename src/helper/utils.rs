use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
pub fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut ema = Vec::new();
    let smoothing_factor = 2.0 / (period as f64 + 1.0);

    // Calculate the initial SMA as the first EMA value
    let mut sma = data[0..period].iter().sum::<f64>() / period as f64;
    ema.push(sma);

    // Calculate EMA for the remaining data
    for i in period..data.len() {
        sma = (data[i] * smoothing_factor) + (ema[i - period] * (1.0 - smoothing_factor));
        ema.push(sma);
    }

    ema
}

pub fn calculate_sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut sma = Vec::new();
    for i in (period - 1)..data.len() {
        let sum: f64 = data[(i - period + 1)..=i].iter().sum();
        sma.push(sum / period as f64);
    }
    sma
}

pub fn smooth(values: &[f64], period: usize) -> Vec<f64> {
    let mut smoothed = Vec::with_capacity(values.len());

    for i in 0..values.len() {
        if i >= period - 1 {
            let window = &values[i + 1 - period..i + 1];
            let sum: f64 = window.iter().sum();
            let avg = sum / period as f64;
            smoothed.push(avg);
        } else {
            smoothed.push(f64::NAN);
        }
    }

    smoothed
}

pub fn highest_lowest(data: &[f64], period: usize, index: usize) -> (f64, f64) {
    let mut highest_high = f64::NEG_INFINITY;
    let mut lowest_low = f64::INFINITY;

    for i in (index + 1 - period)..=index {
        if data[i] > highest_high {
            highest_high = data[i];
        }
        if data[i] < lowest_low {
            lowest_low = data[i];
        }
    }

    (highest_high, lowest_low)
}

#[wasm_bindgen]
pub fn vec_to_js_array(vec: Vec<f64>) -> Result<JsValue, JsValue> {
    // Use serde-wasm-bindgen to serialize the Vec<f64> into a JsValue
    Ok(serde_wasm_bindgen::to_value(&vec)?)
}

pub fn get_f64_from_jsvalue(value: &JsValue) -> f64 {
    value.as_f64().unwrap_or(f64::NAN)
}
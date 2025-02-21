use wasm_bindgen::JsValue;
use crate::create_error;

pub fn calculate_sma(data: &[f64], period: usize) -> Result<Vec<f64>, JsValue> {

    if period == 0 {
        return Err(create_error("[SMA] Period must be greater than 0"));
    }

    if data.is_empty() {
        return Err(create_error("[SMA] Data array cannot be empty"));
    }

    if data.len() < period {
        return Err(create_error(&format!(
            "[SMA] Data array length ({}) is less than period ({})",
            data.len(),
            period
        )));
    }

    let mut sma = Vec::with_capacity(data.len()); // Pré-allocation de la capacité
    let mut sum = 0.0;

    for i in 0..period {
        sum += data[i];
    }
    sma.push(sum / period as f64);

    for i in period..data.len() {
        sum += data[i] - data[i - period];
        sma.push(sum / period as f64);
    }

    let mut result = vec![f64::NAN; period - 1];
    result.extend(sma);

    Ok(result)
}

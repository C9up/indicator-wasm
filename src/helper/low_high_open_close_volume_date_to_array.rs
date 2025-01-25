use serde::Serialize;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct MarketData {
    low: f64,
    high: f64,
    open: f64,
    close: f64,
    volume: f64,
    date: String,
}

#[derive(Serialize)]
struct MarketDataResult {
    low: Vec<f64>,
    high: Vec<f64>,
    open: Vec<f64>,
    close: Vec<f64>,
    volume: Vec<f64>,
    date: Vec<String>,
}

#[wasm_bindgen(js_name = lowHighOpenCloseVolumeDateToArray)]
pub fn low_high_open_close_volume_date_to_array(data: JsValue) -> Result<JsValue, JsValue> {
    // Deserialize the input data into a Vec<MarketData>
    let market_data: Vec<MarketData> = serde_wasm_bindgen::from_value(data)?;

    // Initialize vectors for each type of data
    let mut low = Vec::new();
    let mut high = Vec::new();
    let mut open = Vec::new();
    let mut close = Vec::new();
    let mut volume = Vec::new();
    let mut date = Vec::new();

    // Populate the vectors
    for item in market_data {
        low.push(item.low);
        high.push(item.high);
        open.push(item.open);
        close.push(item.close);
        volume.push(item.volume);
        date.push(item.date);
    }

    // Create the result struct
    let result = MarketDataResult {
        low,
        high,
        open,
        close,
        volume,
        date,
    };

    // Serialize the result struct into a JsValue
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
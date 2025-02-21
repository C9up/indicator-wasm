use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MarketData {
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub date: String, // Assuming date is a string for simplicity
}

#[derive(Serialize, Deserialize)]
pub struct MarketDataResult {
    pub lows: Vec<f64>,
    pub highs: Vec<f64>,
    pub opens: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<f64>,
    pub dates: Vec<String>,
}

pub fn process_market_data(market_data: Vec<MarketData>) -> MarketDataResult {
    let mut lows = Vec::new();
    let mut highs = Vec::new();
    let mut opens = Vec::new();
    let mut closes = Vec::new();
    let mut volumes = Vec::new();
    let mut dates = Vec::new();

    for item in market_data {
        lows.push(item.low);
        highs.push(item.high);
        opens.push(item.open);
        closes.push(item.close);
        volumes.push(item.volume);
        dates.push(item.date);
    }

    MarketDataResult {
        lows,
        highs,
        opens,
        closes,
        volumes,
        dates,
    }
}

#[wasm_bindgen(js_name = lowHighOpenCloseVolumeDateToArray)]
pub fn low_high_open_close_volume_date_to_array(data: JsValue) -> Result<JsValue, JsValue> {
    // Désérialiser les données d'entrée
    let market_data: Vec<MarketData> = serde_wasm_bindgen::from_value(data)
        .map_err(|err| JsValue::from_str(&format!("Failed to deserialize market data: {}", err)))?;

    // Appeler la logique métier
    let result = process_market_data(market_data);

    // Sérialiser le résultat
    serde_wasm_bindgen::to_value(&result)
        .map_err(|err| JsValue::from_str(&format!("Failed to serialize result: {}", err)))
}

pub fn low_high_open_close_volume_date_deserialize(segment: JsValue) -> MarketDataResult {
    let market_data_result: MarketDataResult = serde_wasm_bindgen::from_value(segment).expect("Failed to deserialize market data result");
    market_data_result
}
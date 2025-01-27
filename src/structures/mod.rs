use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MarketData {
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub date: String,
}

#[derive(Serialize)]
pub struct MarketDataResult {
    pub lows: Vec<f64>,
    pub highs: Vec<f64>,
    pub opens: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<f64>,
    pub dates: Vec<String>,
}
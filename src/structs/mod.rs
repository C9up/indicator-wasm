pub mod ichimoku_struct;
pub mod kagi_struct;
pub mod bollinger_bands_struct;
pub mod entry_exit_signals_struct;
pub mod pivot_points_struct;
pub mod extract_important_levels_struct;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MarketData {
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub date: String,
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
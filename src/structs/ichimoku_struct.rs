use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IchimokuResult {
    pub tenkan_sen: Vec<f64>,
    pub kijun_sen: Vec<f64>,
    pub senkou_span_a: Vec<f64>,
    pub senkou_span_b: Vec<f64>,
    pub chikou_span: Vec<f64>,
}
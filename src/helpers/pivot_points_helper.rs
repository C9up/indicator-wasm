pub fn calculate_pivot_point(high: f64, low: f64, close: f64) -> f64 {
    (high + low + close) / 3.0
}
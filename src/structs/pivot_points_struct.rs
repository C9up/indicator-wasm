use crate::helpers::pivot_points_helper::calculate_pivot_point;

#[derive(Debug)]
pub struct PivotLevels {
    pub pivot_point: f64,
    pub resistance1: f64,
    pub resistance2: f64,
    pub support1: f64,
    pub support2: f64,
}

impl PivotLevels {
    // Calculer les niveaux de support et résistance
    pub fn new(high: f64, low: f64, close: f64) -> Self {
        let pivot_point = calculate_pivot_point(high, low, close);
        let range = high - low;

        PivotLevels {
            pivot_point,
            resistance1: (2.0 * pivot_point) - low,
            resistance2: pivot_point + range,
            support1: (2.0 * pivot_point) - high,
            support2: pivot_point - range,
        }
    }
}
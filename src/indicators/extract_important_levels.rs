use crate::serialize_to_js_value;
use crate::structs::extract_important_levels_struct::DataEntry;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl DataEntry {
    pub fn new(
        pivot_point: f64,
        resistance1: f64,
        resistance2: f64,
        support1: f64,
        support2: f64,
    ) -> DataEntry {
        DataEntry {
            pivot_point,
            resistance1,
            resistance2,
            support1,
            support2,
        }
    }
}

#[wasm_bindgen]
pub fn extract_important_levels(data: &[f64]) -> Result<JsValue, JsValue> {
    let mut supports = Vec::new();
    let mut resistances = Vec::new();
    let mut pivot_points = Vec::new();

    const WINDOW: usize = 5;

    for i in WINDOW..(data.len() - WINDOW) {
        let current = data[i];
        let is_highest = (i - WINDOW..i).all(|j| data[j] <= current)
            && (i + 1..=i + WINDOW).all(|j| data[j] <= current);
        let is_lowest = (i - WINDOW..i).all(|j| data[j] >= current)
            && (i + 1..=i + WINDOW).all(|j| data[j] >= current);

        if is_highest {
            resistances.push(current);
        }

        if is_lowest {
            supports.push(current);
        }

        if is_highest || is_lowest {
            pivot_points.push(current);
        }
    }

    let highest_resistance = if !resistances.is_empty() {
        resistances.iter().cloned().fold(f64::MIN, f64::max)
    } else {
        data.iter().cloned().fold(f64::MIN, f64::max)
    };

    let lowest_support = if !supports.is_empty() {
        supports.iter().cloned().fold(f64::MAX, f64::min)
    } else {
        data.iter().cloned().fold(f64::MAX, f64::min)
    };

    let average_pivot = if !pivot_points.is_empty() {
        pivot_points.iter().sum::<f64>() / pivot_points.len() as f64
    } else {
        data.iter().sum::<f64>() / data.len() as f64
    };

    serialize_to_js_value(&(
        highest_resistance,
        lowest_support,
        average_pivot,
        supports,
        resistances,
    ))
}

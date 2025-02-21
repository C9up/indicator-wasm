use crate::calculate_sma_helper::calculate_sma;
use crate::{create_error, jsvalue_to_f64};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simple_moving_average(data: JsValue, period: usize) -> Result<Vec<f64>, JsValue> {
    let vec_data = jsvalue_to_f64(data);

    if period == 0 {
        return Err(create_error("Period must be greater than 0"));
    }
    if vec_data.is_empty() {
        return Err(create_error("Data array cannot be empty"));
    }
    if vec_data.len() < period {
        return Err(create_error(&format!(
            "Data array length ({}) is less than period ({})",
            vec_data.len(),
            period
        )));
    }

    Ok(calculate_sma(&vec_data, period)?)
}

#[cfg(test)]
mod tests {
    use serde_wasm_bindgen::*;
    use super::*;

    // Test pour vérifier la validation du period
    #[test]
    fn test_invalid_period_zero() {
        let data = to_value(&vec![1.0, 2.0, 3.0]).unwrap();
        let result = simple_moving_average(data, 0);
        assert_eq!(result, Err(JsValue::from_str("Period must be greater than 0")));
    }

    // Test pour vérifier la validation du tableau de données vide
    #[test]
    fn test_empty_data() {
        let data = to_value::<Vec<f64>>(&vec![]).unwrap();
        let result = simple_moving_average(data, 2);
        assert_eq!(result, Err(JsValue::from_str("Data array cannot be empty")));
    }

    // Test pour vérifier la validation lorsque la taille des données est inférieure à period
    #[test]
    fn test_data_length_less_than_period() {
        let data = to_value(&vec![1.0, 2.0]).unwrap();
        let result = simple_moving_average(data, 3);
        assert_eq!(result, Err(JsValue::from_str("Data array length (2) is less than period (3)")));
    }

    // Test pour un cas valide avec des données et un period correct
    #[test]
    fn test_valid_input() {
        let data = to_value(&vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        let result = simple_moving_average(data, 3);
        assert_eq!(result.unwrap(), vec![2.0, 3.0, 4.0]);
    }
}
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::deserialize_js_value;
use crate::low_high_open_close_volume_date_to_array_helper::{low_high_open_close_volume_date_to_array, MarketDataResult};
use crate::structs::pivot_points_struct::PivotLevels;

#[wasm_bindgen]
pub fn pivot_points(data: JsValue) -> Result<Vec<f64>, JsValue> {
    // Traitement des données
    let processed_data = low_high_open_close_volume_date_to_array(data)
        .map_err(|_| JsValue::from_str("Failed to process input data"))?;

    let market_data: MarketDataResult = deserialize_js_value(&processed_data)
        .map_err(|_| JsValue::from_str("Failed to deserialize market data"))?;

    let highs = market_data.highs;
    let lows = market_data.lows;
    let closes = market_data.closes;

    let mut result = Vec::new();

    // Utilisation de l'itération combinée
    for (high, low, close) in highs.iter().zip(lows.iter()).zip(closes.iter()).map(|((high, low), close)| (high, low, close)) {
        let pivot_levels = PivotLevels::new(*high, *low, *close);

        // Ajout des résultats dans le vecteur
        result.push(pivot_levels.pivot_point);
        result.push(pivot_levels.resistance1);
        result.push(pivot_levels.resistance2);
        result.push(pivot_levels.support1);
        result.push(pivot_levels.support2);
    }

    Ok(result)
}
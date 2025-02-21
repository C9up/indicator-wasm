use wasm_bindgen::prelude::*;
use thiserror::Error;
use crate::helpers::ichimoku_helper::{average_series, calculate_midline, shift_forward};
use crate::structs::ichimoku_struct::IchimokuResult;
use crate::low_high_open_close_volume_date_to_array_helper::{
    low_high_open_close_volume_date_to_array,
    MarketDataResult
};
use crate::{deserialize_js_value, serialize_to_js_value};
use crate::ichimoku_helper::shift_backward;

#[derive(Error, Debug)]
pub enum IchimokuError {
    #[error("Invalid input data: {0}")]
    InvalidInput(String),
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<IchimokuError> for JsValue {
    fn from(error: IchimokuError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

#[derive(Debug)]
struct IchimokuParams {
    period_tenkan: usize,
    period_kijun: usize,
    period_senkou: usize,
}

impl Default for IchimokuParams {
    fn default() -> Self {
        Self {
            period_tenkan: 9,
            period_kijun: 26,
            period_senkou: 52,
        }
    }
}

impl IchimokuParams {
    fn new(
        period_tenkan: Option<usize>,
        period_kijun: Option<usize>,
        period_senkou: Option<usize>,
    ) -> Result<Self, IchimokuError> {
        let params = Self {
            period_tenkan: period_tenkan.unwrap_or(Self::default().period_tenkan),
            period_kijun: period_kijun.unwrap_or(Self::default().period_kijun),
            period_senkou: period_senkou.unwrap_or(Self::default().period_senkou),
        };

        // Validation des paramètres
        if params.period_tenkan == 0 || params.period_kijun == 0 || params.period_senkou == 0 {
            return Err(IchimokuError::InvalidInput(
                "Periods must be greater than 0".to_string(),
            ));
        }

        Ok(params)
    }
}

fn validate_market_data(market_data: &MarketDataResult, params: &IchimokuParams) -> Result<(), IchimokuError> {
    if market_data.highs.len() != market_data.lows.len() {
        return Err(IchimokuError::InvalidInput(
            "The high and low arrays must have the same length".to_string(),
        ));
    }

    if market_data.highs.is_empty() {
        return Err(IchimokuError::InvalidInput(
            "Input arrays cannot be empty".to_string(),
        ));
    }

    if market_data.highs.len() < params.period_senkou {
        return Err(IchimokuError::InvalidInput(
            format!(
                "Insufficient data: expected at least {} elements, got {}",
                params.period_senkou,
                market_data.highs.len()
            )
        ));
    }

    Ok(())
}

fn calculate_ichimoku_values(
    market_data: &MarketDataResult,
    params: &IchimokuParams,
) -> Result<IchimokuResult, IchimokuError> {
    // Calcul des composants
    let tenkan_sen = calculate_midline(&market_data.highs, &market_data.lows, params.period_tenkan);
    let kijun_sen = calculate_midline(&market_data.highs, &market_data.lows, params.period_kijun);

    // Calcul de Senkou Span A (moyennes de Tenkan et Kijun)
    let senkou_span_a = shift_forward(
        &average_series(&tenkan_sen, &kijun_sen),
        params.period_kijun
    );

    // Calcul de Senkou Span B
    let senkou_span_b = shift_forward(
        &calculate_midline(&market_data.highs, &market_data.lows, params.period_senkou),
        params.period_kijun
    );

    let chikou_span = shift_backward(&market_data.closes, params.period_kijun);

    Ok(IchimokuResult {
        tenkan_sen,
        kijun_sen,
        senkou_span_a,
        senkou_span_b,
        chikou_span
    })
}

#[wasm_bindgen]
pub fn ichimoku(
    data: JsValue,
    period_tenkan: Option<usize>,
    period_kijun: Option<usize>,
    period_senkou: Option<usize>,
) -> Result<JsValue, JsValue> {
    // Initialisation et validation des paramètres
    let params = IchimokuParams::new(period_tenkan, period_kijun, period_senkou)
        .map_err(|e| e.to_string())?;

    // Traitement des données d'entrée
    let processed_data = low_high_open_close_volume_date_to_array(data)
        .map_err(|e| IchimokuError::ProcessingError(e.as_string().unwrap_or_default()))?;

    let market_data: MarketDataResult = deserialize_js_value(&processed_data)
        .map_err(|e| IchimokuError::SerializationError(e.as_string().unwrap_or_default()))?;

    // Validation des données
    validate_market_data(&market_data, &params)?;

    // Calcul des valeurs Ichimoku
    let result = calculate_ichimoku_values(&market_data, &params)?;

    // Sérialisation du résultat
    serialize_to_js_value(&result)
        .map_err(|e| IchimokuError::SerializationError(e.as_string().unwrap_or_default()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ichimoku_params_validation() {
        assert!(IchimokuParams::new(Some(0), Some(26), Some(52)).is_err());
        assert!(IchimokuParams::new(Some(9), Some(0), Some(52)).is_err());
        assert!(IchimokuParams::new(Some(9), Some(26), Some(0)).is_err());

        let valid_params = IchimokuParams::new(Some(9), Some(26), Some(52));
        assert!(valid_params.is_ok());
    }
}
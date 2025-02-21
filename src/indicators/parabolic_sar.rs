use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::low_high_open_close_volume_date_to_array_helper::{low_high_open_close_volume_date_to_array, MarketDataResult};
use crate::{create_error, deserialize_js_value};

/// Calcule le SAR provisoire à partir du SAR précédent, de l'EP et de l'AF
fn compute_new_sar(sar_prev: f64, ep: f64, af: f64) -> f64 {
    sar_prev + af * (ep - sar_prev)
}

/// Applique les bornes du SAR en fonction de la tendance :
/// en tendance haussière, le SAR ne doit pas dépasser les bas des 2 périodes précédentes,
/// en tendance baissière, il ne doit pas être inférieur aux hauts des 2 périodes précédentes.
fn apply_boundaries(is_uptrend: bool, new_sar: f64, highs: &[f64], lows: &[f64], i: usize) -> f64 {
    if is_uptrend {
        let bound = if i >= 2 {
            lows[i - 1].min(lows[i - 2])
        } else {
            lows[i - 1]
        };
        new_sar.min(bound)
    } else {
        let bound = if i >= 2 {
            highs[i - 1].max(highs[i - 2])
        } else {
            highs[i - 1]
        };
        new_sar.max(bound)
    }
}

/// Met à jour l’Extreme Point (EP) et l’Acceleration Factor (AF)
/// uniquement si le nouveau prix extrême est atteint dans la tendance.
fn update_ep_and_af(is_uptrend: bool, current_ep: f64, current_af: f64, high: f64, low: f64, increment: f64, max_value: f64) -> (f64, f64) {
    if is_uptrend {
        if high > current_ep {
            let new_ep = high;
            let new_af = (current_af + increment).min(max_value);
            return (new_ep, new_af);
        }
    } else {
        if low < current_ep {
            let new_ep = low;
            let new_af = (current_af + increment).min(max_value);
            return (new_ep, new_af);
        }
    }
    (current_ep, current_af)
}

#[wasm_bindgen]
pub fn parabolic_sar(
    data: JsValue,
    start: Option<f64>,
    increment: Option<f64>,
    max_value: Option<f64>
) -> Result<Vec<f64>, JsValue> {

    let start = start.unwrap_or(0.02);
    let increment = increment.unwrap_or(0.02);
    let max_value = max_value.unwrap_or(0.2);

    let processed_data = low_high_open_close_volume_date_to_array(data)?;
    let market_data: MarketDataResult = deserialize_js_value(&processed_data)?;
    let highs = market_data.highs;
    let lows = market_data.lows;
    let closes = market_data.closes;

    let len = highs.len();
    if len < 2 {
        return Err(create_error("Not enough data."))
    }

    let mut sar = vec![0.0; len];
    let mut af = start;
    let mut is_uptrend = true;
    // Pour démarrer, on part d'une tendance haussière et on fixe l’EP au premier high
    let mut ep = highs[0];

    // Initialisation du SAR sur la première période (ici, la clôture)
    sar[0] = closes[0];

    for i in 1..len {
        // Calcul du SAR provisoire
        let provisional_sar = compute_new_sar(sar[i - 1], ep, af);
        let bounded_sar = apply_boundaries(is_uptrend, provisional_sar, &highs, &lows, i);

        // Vérification du renversement de tendance
        if is_uptrend && lows[i] < bounded_sar {
            // Passage d'une tendance haussière à une tendance baissière
            is_uptrend = false;
            sar[i] = ep;      // Le SAR est fixé à l'ancien EP (le plus haut atteint)
            ep = lows[i];     // Nouvel EP : le plus bas actuel
            af = start;  // Réinitialisation de l’AF
        } else if !is_uptrend && highs[i] > bounded_sar {
            // Passage d'une tendance baissière à une tendance haussière
            is_uptrend = true;
            sar[i] = ep;      // Le SAR est fixé à l'ancien EP (le plus bas atteint)
            ep = highs[i];    // Nouvel EP : le plus haut actuel
            af = start;  // Réinitialisation de l’AF
        } else {
            // Tendance ininterrompue : on conserve le SAR calculé et on met à jour l'EP et l'AF si nécessaire
            sar[i] = bounded_sar;
            let (new_ep, new_af) = update_ep_and_af(is_uptrend, ep, af, highs[i], lows[i], increment, max_value);
            ep = new_ep;
            af = new_af;
        }
    }

    Ok(sar)
}

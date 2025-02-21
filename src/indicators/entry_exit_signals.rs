use crate::helpers::calculate_atr_helper::calculate_atr;
use crate::helpers::calculate_ema_helper::calculate_ema;
use crate::helpers::calculate_sma_helper::calculate_sma;
use crate::helpers::entry_exit_signals_helper::{is_entry_signal, is_exit_signal};
use crate::jsvalue_to_f64;
use crate::structs::entry_exit_signals_struct::Signal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn entry_exit_signals(
    data: JsValue,
    sma_period: usize,
    ema_period: usize,
    atr_period: usize,
    threshold: f64,
) -> Vec<Signal> {
    let prices = jsvalue_to_f64(data);
    if prices.len() < sma_period + ema_period + atr_period {
        return Vec::new();
    }

    let mut signals = Vec::new();

    let sma_values = calculate_sma(&prices, sma_period).unwrap(); // SMA
    let ema_values = calculate_ema(&prices, ema_period); // EMA
    let atr_values = calculate_atr(&prices, atr_period); // ATR

    let mut trend_up = false;

    // Corrected indexing for indicator values.  Crucially, you need to account for the fact
    // that the indicators have fewer values than the original price series.
    for i in atr_period..prices.len() {
        let current_price = prices[i];

        // Accessing the indicator values needs to be offset.
        let current_sma_index = i - sma_period + 1; // +1 is crucial.  SMA starts from period 1
        let current_ema_index = i - ema_period + 1; // +1 is crucial. EMA starts from period 1
        let current_atr_index = i - atr_period;

        if current_sma_index >= sma_values.len()
            || current_ema_index >= ema_values.len()
            || current_atr_index >= atr_values.len()
        {
            continue; // Skip if index is out of bounds due to calculation lag.
        }

        let current_sma = sma_values[current_sma_index];
        let current_ema = ema_values[current_ema_index];
        let current_atr = atr_values[current_atr_index];

        // Calcul des seuils d'entrée et de sortie avec l'ATR
        let entry_threshold = current_sma + current_atr * threshold;
        let exit_threshold = current_sma - current_atr * threshold;

        // Logique pour l'entrée et la sortie en utilisant SMA, EMA et ATR
        if is_entry_signal(
            current_price,
            current_sma,
            current_ema,
            entry_threshold,
            trend_up,
        ) {
            signals.push(Signal::new(0, current_price, i));
            trend_up = true;
        } else if is_exit_signal(
            current_price,
            current_sma,
            current_ema,
            exit_threshold,
            trend_up,
        ) {
            signals.push(Signal::new(1, current_price, i));
            trend_up = false;
        }
    }

    signals
}

use wasm_bindgen::prelude::*;
use crate::utils::smooth;

#[wasm_bindgen]
pub struct DMIResult {
    plus_di: Vec<f64>,
    minus_di: Vec<f64>,
    adx: Vec<f64>,
}


#[wasm_bindgen]
pub struct DirectionalMovementIndex {
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>
}
#[wasm_bindgen]
impl DirectionalMovementIndex {
    #[wasm_bindgen(constructor)]
    pub fn new(highs: Vec<f64>, lows: Vec<f64>, closes: Vec<f64>) -> Self {
        DirectionalMovementIndex {
            highs,
            lows,
            closes,
        }
    }

    pub fn period(&mut self, period: usize) -> DMIResult {

        let len = self.highs.len();
        if len < period {
            return DMIResult {
                plus_di: Vec::new(),
                minus_di: Vec::new(),
                adx: Vec::new(),
            };
        }

        let mut tr = vec![0.0; len]; // True Range
        let mut plus_dm = vec![0.0; len]; // +DM
        let mut minus_dm = vec![0.0; len]; // -DM

        // Calculate True Range, +DM, and -DM
        for i in 1..len {
            let high = self.highs[i];
            let low = self.lows[i];
            let prev_close = self.closes[i - 1];

            tr[i] = (high - low).max((high - prev_close).abs()).max((low - prev_close).abs());

            let up_move = high - self.highs[i - 1];
            let down_move = self.lows[i - 1] - low;

            if up_move > down_move && up_move > 0.0 {
                plus_dm[i] = up_move;
            }
            if down_move > up_move && down_move > 0.0 {
                minus_dm[i] = down_move;
            }
        }

        // Smooth +DM, -DM, and TR
        let smoothed_plus_dm = smooth(&plus_dm, period);
        let smoothed_minus_dm = smooth(&minus_dm, period);
        let smoothed_tr = smooth(&tr, period);

        // Calculate +DI and -DI
        let mut plus_di = vec![0.0; len];
        let mut minus_di = vec![0.0; len];

        for i in period..len {
            if smoothed_tr[i] > 0.0 {
                plus_di[i] = 100.0 * (smoothed_plus_dm[i] / smoothed_tr[i]);
                minus_di[i] = 100.0 * (smoothed_minus_dm[i] / smoothed_tr[i]);
            }
        }

        // Calculate ADX
        let mut dx = vec![0.0; len]; // Directional Movement Index
        for i in period..len {
            if plus_di[i] + minus_di[i] > 0.0 {
                dx[i] = 100.0 * ((plus_di[i] - minus_di[i]).abs() / (plus_di[i] + minus_di[i]));
            }
        }

        let adx = smooth(&dx, period);

        DMIResult {
            plus_di,
            minus_di,
            adx,
        }
    }
}
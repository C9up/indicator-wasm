
pub fn true_range(high: &[f64], low: &[f64], close: &[f64], index: usize) -> f64 {
    let tr1 = high[index] - low[index];
    let tr2 = (high[index] - close[index - 1]).abs();
    let tr3 = (low[index] - close[index - 1]).abs();
    tr1.max(tr2).max(tr3)
}

pub fn directional_movement(high: &[f64], low: &[f64], index: usize) -> (f64, f64) {
    let up_move = high[index] - high[index - 1];
    let down_move = low[index - 1] - low[index];
    let plus_dm = if up_move > down_move && up_move > 0.0 { up_move } else { 0.0 };
    let minus_dm = if down_move > up_move && down_move > 0.0 { down_move } else { 0.0 };
    (plus_dm, minus_dm)
}
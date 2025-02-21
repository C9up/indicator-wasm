pub fn calculate_midline(highs: &[f64], lows: &[f64], period: usize) -> Vec<f64> {
    if highs.is_empty() || lows.is_empty() || period == 0 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(highs.len());

    // Remplir les périodes initiales avec NaN
    for _ in 0..period-1 {
        result.push(f64::NAN);
    }

    // Calcul des points médians
    for i in period-1..highs.len() {
        let start = i + 1 - period;
        let window_high = &highs[start..=i];
        let window_low = &lows[start..=i];
        let max_high = window_high.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_low = window_low.iter().cloned().fold(f64::INFINITY, f64::min);
        result.push((max_high + min_low) / 2.0);
    }

    result
}
pub fn average_series(series1: &[f64], series2: &[f64]) -> Vec<f64> {
    let min_len = series1.len().min(series2.len());
    (0..min_len)
        .map(|i| {
            if series1[i].is_nan() || series2[i].is_nan() {
                f64::NAN
            } else {
                (series1[i] + series2[i]) / 2.0
            }
        })
        .collect()
}

pub fn shift_forward(series: &[f64], shift: usize) -> Vec<f64> {
    let mut result = vec![f64::NAN; shift];
    result.extend_from_slice(&series[..series.len().saturating_sub(shift)]);
    result
}

pub fn shift_backward(series: &[f64], shift: usize) -> Vec<f64> {
    let mut result = series[shift..].to_vec();
    result.extend(std::iter::repeat(f64::NAN).take(shift));
    result
}
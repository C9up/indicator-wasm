pub fn abs_diff(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .collect()
}

pub fn max_in_window(data: &[f64], period: usize) -> Vec<f64> {
    data.windows(period)
        .map(|w| w.iter().fold(f64::MIN, |a, &b| a.max(b)))
        .collect()
}

pub fn min_in_window(data: &[f64], period: usize) -> Vec<f64> {
    data.windows(period)
        .map(|w| w.iter().fold(f64::MAX, |a, &b| a.min(b)))
        .collect()
}
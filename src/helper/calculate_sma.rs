pub fn calculate_sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut sma = Vec::new();
    for i in (period - 1)..data.len() {
        let sum: f64 = data[(i - period + 1)..=i].iter().sum();
        sma.push(sum / period as f64);
    }
    sma
}
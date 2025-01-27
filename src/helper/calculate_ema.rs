pub fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut ema = Vec::new();
    let smoothing_factor = 2.0 / (period as f64 + 1.0);

    // Calculate the initial SMA as the first EMA value
    let mut sma = data[0..period].iter().sum::<f64>() / period as f64;
    ema.push(sma);

    // Calculate EMA for the remaining data
    for i in period..data.len() {
        sma = (data[i] * smoothing_factor) + (ema[i - period] * (1.0 - smoothing_factor));
        ema.push(sma);
    }

    ema
}
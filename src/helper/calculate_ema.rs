pub fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {

    if period == 1 {
        // Bypass EMA smoothing and return the raw data.
        return data.to_vec();
    }

    // If there is not enough data to compute the initial SMA, return a vector of NaNs.
    if data.len() < period {
        return vec![f64::NAN; data.len()];
    }

    // Calculate the smoothing factor.
    let smoothing_factor = 2.0 / (period as f64 + 1.0);
    let mut ema = Vec::with_capacity(data.len());

    // For indices where EMA cannot be computed, fill with NaN.
    for _ in 0..(period - 1) {
        ema.push(f64::NAN);
    }

    // Calculate the initial SMA (simple moving average) over the first 'period' values.
    let initial_sma: f64 = data[..period].iter().sum::<f64>() / period as f64;
    ema.push(initial_sma);

    // Use the initial SMA as the first EMA value.
    let mut previous_ema = initial_sma;
    // Compute EMA for each subsequent data point.
    for i in period..data.len() {
        let current_ema = data[i] * smoothing_factor + previous_ema * (1.0 - smoothing_factor);
        ema.push(current_ema);
        previous_ema = current_ema;
    }

    ema
}

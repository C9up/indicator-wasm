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

    // Initialisation avec la première valeur
    ema.push(data[0]);

    // Calcul récursif sans SMA initiale
    for i in 1..data.len() {
        let val = smoothing_factor * data[i] + (1.0 - smoothing_factor) * ema[i - 1];
        ema.push(val);
    }

    ema
}

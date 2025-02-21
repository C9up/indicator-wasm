pub fn calculate_atr(prices: &[f64], period: usize) -> Vec<f64> {
    let mut atr_values = Vec::new();

    if prices.len() < period + 1 {
        return atr_values; // Pas assez de données pour calculer l'ATR
    }

    let mut tr_values = Vec::new();

    // Calcul du True Range (TR) pour chaque période
    for i in 1..prices.len() {
        let current_price = prices[i];
        let _previous_price = prices[i - 1];

        let high = current_price;
        let low = current_price;
        let prev_close = prices[i - 1];

        let tr = f64::max(
            f64::max(high - low, (high - prev_close).abs()),
            (low - prev_close).abs(),
        );
        tr_values.push(tr);
    }

    // Calcul de l'ATR sur la période donnée en utilisant une moyenne mobile
    for i in period..tr_values.len() {
        let sum: f64 = tr_values[i - period..i].iter().sum();
        let atr = sum / period as f64;
        atr_values.push(atr);
    }

    atr_values
}

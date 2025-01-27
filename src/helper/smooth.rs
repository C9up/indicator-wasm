pub fn smooth(values: &[f64], period: usize) -> Vec<f64> {
    let mut smoothed = Vec::with_capacity(values.len());

    for i in 0..values.len() {
        if i >= period - 1 {
            let window = &values[i + 1 - period..i + 1];
            let sum: f64 = window.iter().sum();
            let avg = sum / period as f64;
            smoothed.push(avg);
        } else {
            smoothed.push(f64::NAN);
        }
    }

    smoothed
}
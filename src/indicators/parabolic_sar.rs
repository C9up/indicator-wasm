use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParabolicSar {
    high_prices: Vec<f64>,
    low_prices: Vec<f64>
}

#[wasm_bindgen]
impl ParabolicSar {

    #[wasm_bindgen(constructor)]
    pub fn new(high_prices: Vec<f64>, low_prices: Vec<f64>) -> Self {
        ParabolicSar {
            high_prices,
            low_prices,
        }
    }

    pub fn period(&mut self, mut acceleration_factor: f64, max_acceleration_factor: f64, is_long: Option<bool>, extreme_point: Option<f64>) -> Vec<f64> {
        let mut is_long = is_long.unwrap_or(false);
        let mut extreme_point = extreme_point.unwrap_or(0.0);
        let len = self.high_prices.len();
        let mut sar_values = Vec::new();

        if len < 2 {
            return vec![]; // Not enough data to calculate Parabolic SAR
        }

        // Initialize the first SAR value
        let mut sar = if is_long {
            self.low_prices[0] // Start with the first low for a long position
        } else {
            self.high_prices[0] // Start with the first high for a short position
        };
        sar_values.push(sar);

        // Initialize the extreme point
        extreme_point = if is_long {
            self.high_prices[0]
        } else {
            self.low_prices[0]
        };

        // Calculate SAR for the remaining data points
        for i in 1..len {
            let high = self.high_prices[i];
            let low = self.low_prices[i];

            // Update SAR value
            sar = sar + acceleration_factor * (extreme_point - sar);

            // Check for SAR reversal
            if is_long {
                if low < sar {
                    // Switch to short position
                    is_long = false;
                    sar = extreme_point; // Reset SAR to the extreme point
                    extreme_point = low; // Update extreme point
                    acceleration_factor = 0.02; // Reset acceleration factor
                } else {
                    // Update extreme point and acceleration factor
                    if high > extreme_point {
                        extreme_point = high;
                        acceleration_factor = (acceleration_factor + 0.02).min(max_acceleration_factor);
                    }
                }
            } else {
                if high > sar {
                    // Switch to long position
                    is_long = true;
                    sar = extreme_point; // Reset SAR to the extreme point
                    extreme_point = high; // Update extreme point
                    acceleration_factor = 0.02; // Reset acceleration factor
                } else {
                    // Update extreme point and acceleration factor
                    if low < extreme_point {
                        extreme_point = low;
                        acceleration_factor = (acceleration_factor + 0.02).min(max_acceleration_factor);
                    }
                }
            }

            // Ensure SAR is within the current high-low range
            if is_long {
                sar = sar.min(self.low_prices[i - 1]).min(low);
            } else {
                sar = sar.max(self.high_prices[i - 1]).max(high);
            }

            sar_values.push(sar);
        }

        sar_values
    }
}
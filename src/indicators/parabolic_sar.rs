use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParabolicSar {
    acceleration_factor: f64,
    max_acceleration_factor: f64,
    high_prices: Vec<f64>,
    low_prices: Vec<f64>,
    sar_values: Vec<f64>,
    is_long: bool,
    extreme_point: f64,
}

#[wasm_bindgen]
impl ParabolicSar {
    pub fn new(acceleration_factor: f64, max_acceleration_factor: f64, high_prices: Vec<f64>, low_prices: Vec<f64>) -> Self {
        ParabolicSar {
            acceleration_factor,
            max_acceleration_factor,
            high_prices,
            low_prices,
            sar_values: Vec::new(),
            is_long: true, // Start with a long position
            extreme_point: 0.0,
        }
    }

    pub fn calculate(&mut self) {
        let len = self.high_prices.len();
        if len < 2 {
            return; // Not enough data to calculate Parabolic SAR
        }

        // Initialize the first SAR value
        let mut sar = if self.is_long {
            self.low_prices[0] // Start with the first low for a long position
        } else {
            self.high_prices[0] // Start with the first high for a short position
        };
        self.sar_values.push(sar);

        // Initialize the extreme point
        self.extreme_point = if self.is_long {
            self.high_prices[0]
        } else {
            self.low_prices[0]
        };

        // Calculate SAR for the remaining data points
        for i in 1..len {
            let high = self.high_prices[i];
            let low = self.low_prices[i];

            // Update SAR value
            sar = sar + self.acceleration_factor * (self.extreme_point - sar);

            // Check for SAR reversal
            if self.is_long {
                if low < sar {
                    // Switch to short position
                    self.is_long = false;
                    sar = self.extreme_point; // Reset SAR to the extreme point
                    self.extreme_point = low; // Update extreme point
                    self.acceleration_factor = 0.02; // Reset acceleration factor
                } else {
                    // Update extreme point and acceleration factor
                    if high > self.extreme_point {
                        self.extreme_point = high;
                        self.acceleration_factor = (self.acceleration_factor + 0.02).min(self.max_acceleration_factor);
                    }
                }
            } else {
                if high > sar {
                    // Switch to long position
                    self.is_long = true;
                    sar = self.extreme_point; // Reset SAR to the extreme point
                    self.extreme_point = high; // Update extreme point
                    self.acceleration_factor = 0.02; // Reset acceleration factor
                } else {
                    // Update extreme point and acceleration factor
                    if low < self.extreme_point {
                        self.extreme_point = low;
                        self.acceleration_factor = (self.acceleration_factor + 0.02).min(self.max_acceleration_factor);
                    }
                }
            }

            // Ensure SAR is within the current high-low range
            if self.is_long {
                sar = sar.min(self.low_prices[i - 1]).min(low);
            } else {
                sar = sar.max(self.high_prices[i - 1]).max(high);
            }

            self.sar_values.push(sar);
        }
    }

    pub fn get_sar_values(&self) -> Vec<f64> {
        self.sar_values.clone()
    }
}
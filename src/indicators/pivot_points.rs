use wasm_bindgen::prelude::*;
use crate::calculate_ema::calculate_ema;
use crate::low_high_open_close_volume_date_to_array::{low_high_open_close_volume_date_deserialize, low_high_open_close_volume_date_to_array};

#[wasm_bindgen]
pub struct CongestionZone {
    pub start: f64,
    pub end: f64,
    pub volume_weight: f64,
}

#[wasm_bindgen]
pub struct PivotPoints {
    pivot: f64,
    resistance1: f64,
    resistance2: f64,
    support1: f64,
    support2: f64,
    congestion_zones: Vec<CongestionZone>, // Privé, non exposé directement
}

#[wasm_bindgen]
impl PivotPoints {
    #[wasm_bindgen(getter)]
    pub fn pivot(&self) -> f64 {
        self.pivot
    }

    #[wasm_bindgen(getter)]
    pub fn resistance1(&self) -> f64 {
        self.resistance1
    }

    #[wasm_bindgen(getter)]
    pub fn resistance2(&self) -> f64 {
        self.resistance2
    }

    #[wasm_bindgen(getter)]
    pub fn support1(&self) -> f64 {
        self.support1
    }

    #[wasm_bindgen(getter)]
    pub fn support2(&self) -> f64 {
        self.support2
    }

    #[wasm_bindgen(getter)]
    pub fn congestion_zones(&self) -> js_sys::Array {
        self.congestion_zones
            .iter()
            .map(|zone| {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"start".into(), &zone.start.into()).unwrap();
                js_sys::Reflect::set(&obj, &"end".into(), &zone.end.into()).unwrap();
                js_sys::Reflect::set(&obj, &"volumeWeight".into(), &zone.volume_weight.into()).unwrap();
                JsValue::from(obj) // Conversion explicite en JsValue
            })
            .collect()
    }
}

#[wasm_bindgen]
pub struct Pivot {
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
    volumes: Vec<f64>, // Ajout des volumes pour le calcul des zones de congestion
}

#[wasm_bindgen]
impl Pivot {
    #[wasm_bindgen(constructor)]
    pub fn new(prices: JsValue) -> Self {
        // Convertir les données JavaScript en vecteurs Rust
        let segment = low_high_open_close_volume_date_deserialize(low_high_open_close_volume_date_to_array(prices)
            .expect("Failed to convert market data"));

        Pivot {
            highs: segment.highs,
            lows: segment.lows,
            closes: segment.closes,
            volumes: segment.volumes, // Inclure les volumes
        }
    }

    // Fonction utilitaire pour trouver la valeur maximale dans une tranche de f64
    fn find_max(prices: &[f64]) -> Option<f64> {
        prices.iter().fold(None, |max, &x| {
            if let Some(max_val) = max {
                if x > max_val {
                    Some(x)
                } else {
                    Some(max_val)
                }
            } else {
                Some(x)
            }
        })
    }

    // Fonction utilitaire pour trouver la valeur minimale dans une tranche de f64
    fn find_min(prices: &[f64]) -> Option<f64> {
        prices.iter().fold(None, |min, &x| {
            if let Some(min_val) = min {
                if x < min_val {
                    Some(x)
                } else {
                    Some(min_val)
                }
            } else {
                Some(x)
            }
        })
    }

    // Fonction pour détecter les zones de congestion avec un seuil dynamique et une pondération par volume
    fn detect_congestion_zones(
        prices: &[f64],
        volumes: &[f64],
        volatility_threshold: f64,
    ) -> Vec<CongestionZone> {
        let mut zones = Vec::new();
        let mut start = prices[0];
        let mut end = prices[0];
        let mut volume_sum = volumes[0]; // Suivre le volume pour la zone

        // Calculer un seuil dynamique basé sur la volatilité des prix
        let price_range = Self::find_max(prices).unwrap_or(0.0) - Self::find_min(prices).unwrap_or(0.0);
        let dynamic_threshold = price_range * volatility_threshold;

        // Parcourir les prix pour identifier les zones de congestion
        for i in 1..prices.len() {
            if (prices[i] - end).abs() <= dynamic_threshold {
                end = prices[i]; // Étendre la zone de congestion
                volume_sum += volumes[i]; // Accumuler le volume pour la zone
            } else {
                if start != end {
                    // Ajouter la zone détectée avec son poids de volume
                    zones.push(CongestionZone {
                        start,
                        end,
                        volume_weight: volume_sum,
                    });
                }
                start = prices[i];
                end = prices[i];
                volume_sum = volumes[i]; // Réinitialiser le volume pour la nouvelle zone
            }
        }

        // Ajouter la dernière zone si elle existe
        if start != end {
            zones.push(CongestionZone {
                start,
                end,
                volume_weight: volume_sum,
            });
        }

        zones
    }

    // Fonction principale pour calculer les points pivots, supports, résistances et zones de congestion
    pub fn calculate(&self) -> PivotPoints {
        // Calculer les EMAs pour les hauts, les bas et les clôtures
        let period = self.highs.len(); // Utiliser la longueur des données comme période
        let ema_highs = calculate_ema(&self.highs, period);
        let ema_lows = calculate_ema(&self.lows, period);
        let ema_closes = calculate_ema(&self.closes, period);

        // Utiliser les dernières valeurs EMA pour les calculs
        let ema_high = ema_highs.last().unwrap_or(&0.0);
        let ema_low = ema_lows.last().unwrap_or(&0.0);
        let ema_close = ema_closes.last().unwrap_or(&0.0);

        // Calculer le point pivot et les niveaux de support/résistance
        let pivot = (ema_high + ema_low + ema_close) / 3.0;
        let resistance1 = (2.0 * pivot) - ema_low;
        let support1 = (2.0 * pivot) - ema_high;
        let resistance2 = pivot + (ema_high - ema_low);
        let support2 = pivot - (ema_high - ema_low);

        // Détecter les zones de congestion en utilisant toutes les données de prix et les volumes
        let all_prices = self
            .highs
            .iter()
            .chain(self.lows.iter())
            .chain(self.closes.iter())
            .cloned()
            .collect::<Vec<f64>>();
        let congestion_zones = Self::detect_congestion_zones(&all_prices, &self.volumes, 0.01); // Seuil de volatilité de 1%

        // Retourner les résultats
        PivotPoints {
            pivot,
            resistance1,
            resistance2,
            support1,
            support2,
            congestion_zones,
        }
    }
}
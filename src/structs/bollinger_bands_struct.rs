use serde::{Deserialize, Serialize};

/// Structure de résultat qui sera convertie en objet JS (avec 3 propriétés : middle, upper et lower).
#[derive(Serialize, Deserialize)]
pub struct BollingerBandsResult {
    /// Bande centrale (moyenne mobile)
    pub middle: Vec<f64>,
    /// Bande supérieure (moyenne mobile + k × écart-type)
    pub upper: Vec<f64>,
    /// Bande inférieure (moyenne mobile - k × écart-type)
    pub lower: Vec<f64>,
}
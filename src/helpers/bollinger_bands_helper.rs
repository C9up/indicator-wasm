use crate::bollinger_bands_struct::BollingerBandsResult;

/// Fonction helper qui calcule les Bollinger Bands à partir d'un slice de f64.
///
/// On utilise ici un algorithme à fenêtre glissante pour éviter de recalculer
/// la somme et la somme des carrés à chaque fenêtre.
///
/// Retourne une instance de `BollingerBandsResult` contenant 3 vecteurs.
pub fn compute_bollinger_bands(prices: &[f64], period: usize, multiplier: f64) -> BollingerBandsResult {
    let n = period as f64;
    // Pré-allouer la taille des vecteurs de sortie pour éviter des réallocations
    let out_size = prices.len() - period + 1;
    let mut middle = Vec::with_capacity(out_size);
    let mut upper = Vec::with_capacity(out_size);
    let mut lower = Vec::with_capacity(out_size);

    // Calculer la somme et la somme des carrés pour la première fenêtre
    let (mut sum, mut sum_sq) = prices[..period].iter().fold((0.0, 0.0), |(s, s_sq), &p| {
        (s + p, s_sq + p * p)
    });

    // Calculer la moyenne et l’écart-type pour la fenêtre initiale
    let ma = sum / n;
    let stdev = calculate_std(sum, sum_sq, n);
    middle.push(ma);
    upper.push(ma + multiplier * stdev);
    lower.push(ma - multiplier * stdev);

    // Parcourir le reste des données en glissant la fenêtre
    for i in period..prices.len() {
        let old = prices[i - period];
        let new = prices[i];

        // Mettre à jour la somme et la somme des carrés de la fenêtre
        sum = sum - old + new;
        sum_sq = sum_sq - old * old + new * new;

        let ma = sum / n;
        let stdev = calculate_std(sum, sum_sq, n);
        middle.push(ma);
        upper.push(ma + multiplier * stdev);
        lower.push(ma - multiplier * stdev);
    }

    BollingerBandsResult { middle, upper, lower }
}

/// Fonction helper pour calculer l'écart-type à partir de la somme et la somme des carrés.
/// La variance est calculée selon la formule : variance = (sum_sq - (sum²)/n) / n
#[inline(always)]
fn calculate_std(sum: f64, sum_sq: f64, n: f64) -> f64 {
    let variance = (sum_sq - (sum * sum) / n) / n;
    variance.sqrt()
}
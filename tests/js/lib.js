

export const generateTestData = (days) => {
    let previousClose = 100; // Prix de départ arbitraire
    let previousVolume = 1000; // Volume de départ arbitraire

    return Array.from({ length: days }, (_, i) => {
        const date = new Date(2024, 0, i + 1).toISOString().split('T')[0];

        // Détermine l'ouverture proche de la clôture précédente avec une petite variation
        const open = previousClose + (Math.random() * 2 - 1); // ±1 autour de la clôture précédente

        // Détermine le prix haut et bas du jour avec une variation cohérente
        const high = open + Math.random() * 5;  // Jusqu'à +5% de variation
        const low = open - Math.random() * 5;   // Jusqu'à -5% de variation

        // La clôture est entre le low et le high
        const close = low + Math.random() * (high - low);

        // Volume aléatoire, avec variation de ±50% par rapport au volume précédent
        const volume = Math.round(previousVolume * (0.5 + Math.random()));

        // Mise à jour pour la prochaine itération
        previousClose = close;
        previousVolume = volume;

        return { date, open, high, low, close, volume };
    })
}
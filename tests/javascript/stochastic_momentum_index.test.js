import { test } from '@japa/runner'
import wasmModule from '../../dist/index.js'

test.group('StochasticMomentumIndex', async (group) => {

    let StochasticMomentumIndex
    let prices
    let smi

    group.setup(async () => {
        const module = await wasmModule
        StochasticMomentumIndex = module.StochasticMomentumIndex

        prices = [
            { high: 2,   low: 1,   close: 1.5, open: 1.2, volume: 1000, date: "2025-01-01" },
            { high: 2.5, low: 1.2, close: 2,   open: 1.5, volume: 1100, date: "2025-01-02" },
            { high: 3,   low: 1.4, close: 2.5, open: 2,   volume: 1200, date: "2025-01-03" },
            { high: 2.8, low: 1.3, close: 2.2, open: 2.5, volume: 1150, date: "2025-01-04" },
            { high: 3.2, low: 1.5, close: 2.8, open: 2.2, volume: 1300, date: "2025-01-05" },
        ]

        // Create an instance of StochasticMomentumIndex with the provided market data
        smi = new StochasticMomentumIndex(prices)
    })

    test('should return NaN for indices with insufficient data', async ({ assert }) => {
        // Define the period parameters
        const period_l = 3
        const period_h = 3
        const smoothing_period = 1

        // Calculate the SMI values
        const result = smi.period(period_l, period_h, smoothing_period)

        // The first two values should be NaN because there isn't enough data for computation
        assert.isTrue(isNaN(result[0]), "Expected the first value to be NaN due to insufficient data")
        assert.isTrue(isNaN(result[1]), "Expected the second value to be NaN due to insufficient data")
    })

    test('should compute correct SMI for index 2', async ({ assert }) => {
        const period_l = 3
        const period_h = 3
        const smoothing_period = 1
        const result = smi.period(period_l, period_h, smoothing_period)
        const resultArray = Array.from(result)

        // For index 2 (with period_l = period_h = 3):
        //   - Highest from indices [0..2]: max(2, 2.5, 3) = 3
        //   - Lowest from indices [0..2]: min(1, 1.2, 1.4) = 1
        //   - Midpoint = (3 + 1) / 2 = 2
        //   - Diff = 2.5 - 2 = 0.5, Range = 3 - 1 = 2
        //   - Raw SMI = (0.5 / (2/2)) * 100 = 50
        assert.approximately(resultArray[2], 50, 0.001, "Index 2 SMI value should be approximately 50");
    })

    test('should compute correct SMI for index 3', async ({ assert }) => {
        // Define the period parameters
        const period_l = 3
        const period_h = 3
        const smoothing_period = 1

        // Calculate the SMI values
        const result = smi.period(period_l, period_h, smoothing_period)
        const resultArray = Array.from(result)

        // For index 3:
        //   - Highest from indices [1..3]: max(2.5, 3, 2.8) = 3
        //   - Lowest from indices [1..3]: min(1.2, 1.4, 1.3) = 1.2
        //   - Midpoint = (3 + 1.2) / 2 = 2.1
        //   - diff = 2.2 - 2.1 = 0.1
        //   - range = 3 - 1.2 = 1.8
        //   - SMI = (0.1 / (1.8 / 2)) * 100 ≈ 11.1111
        assert.approximately(resultArray[3], 11.111111111111112, 0.0001, "Index 3 SMI value should be approximately 11.1111")
    })

    test('should compute correct SMI for index 4', async ({ assert }) => {
        // Define the period parameters
        const period_l = 3
        const period_h = 3
        const smoothing_period = 1

        // Calculate the SMI values
        const result = smi.period(period_l, period_h, smoothing_period)
        const resultArray = Array.from(result)

        // For index 4:
        //   - Highest from indices [2..4]: max(3, 2.8, 3.2) = 3.2
        //   - Lowest from indices [2..4]: min(1.4, 1.3, 1.5) = 1.3
        //   - Midpoint = (3.2 + 1.3) / 2 = 2.25
        //   - diff = 2.8 - 2.25 = 0.55
        //   - range = 3.2 - 1.3 = 1.9
        //   - SMI = (0.55 / (1.9 / 2)) * 100 ≈ 57.8947
        assert.approximately(resultArray[4], 57.89473684210527, 0.0001, "Index 4 SMI value should be approximately 57.8947")
    })
})

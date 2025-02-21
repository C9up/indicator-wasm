import { test } from '@japa/runner'
import { generateTestData } from './lib.js';
import {kagiChart, simpleMovingAverage} from "../../dist/index.js";

test.group('Kagi Chart', () => {

    test('should generate the correct Kagi chart data for a given price series', ({ assert }) => {
        // Generating random test data for 30 days
        const testData = generateTestData(30);

        // Extracting only the 'close' prices for Kagi chart input
        const prices = testData.map(item => item.close);

        // Define a reversal amount (e.g., 1.0)
        const reversalAmount = 1.0;

        // Call the wasm function
        const result = kagiChart(prices, reversalAmount);

        // Assert that the result contains prices and directions
        assert.exists(result.prices)
        assert.exists(result.directions)
        assert.isAbove(result.prices.length, 0)
        assert.isAbove(result.directions.length, 0)

        // Additional checks for direction values
        assert.includeDeepMembers(result.directions, ['Yang', 'Yin'])
    });

    test('should handle an empty price list gracefully', ({ assert }) => {
        try {
            kagiChart([], 1.0);
            assert.fail()
        } catch (error) {
            assert.equal(error.message, 'Prices vector must not be empty.')
        }
    });

    test('should handle invalid reversal amount', ({ assert }) => {
        const testData = generateTestData(30);
        const prices = testData.map(item => item.close);

        try {
            kagiChart(prices, -1.0);
            assert.fail()
        } catch (error) {
            assert.equal(error.message, 'Reversal amount must be greater than 0.')
        }
    });
});

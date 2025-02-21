import { test } from '@japa/runner'
import { ichimoku, lowHighOpenCloseVolumeDateToArray } from '../../dist/index.js'
import { generateTestData } from './lib.js'

test.group('Ichimoku Cloud Indicator', () => {

    test('should calculate basic Ichimoku values', ({ assert }) => {
        // Generate 100 days of data to ensure enough points for all calculations
        const testData = generateTestData(100)
        const result = ichimoku(testData)

        assert.properties(result, [
            'tenkan_sen',
            'kijun_sen',
            'senkou_span_a',
            'senkou_span_b',
            'chikou_span'
        ])

        // Check array lengths
        assert.equal(result.tenkan_sen.length, testData.length)
        assert.equal(result.kijun_sen.length, testData.length)
        assert.equal(result.senkou_span_a.length, testData.length)
        assert.equal(result.senkou_span_b.length, testData.length)
        assert.equal(result.chikou_span.length, testData.length)
    })

    test('should handle minimal data set', ({ assert }) => {
        const minimalData = generateTestData(53) // Minimum required for 52-period calculations
        const result = ichimoku(minimalData)

        // Verify initial values are NaN
        assert.isTrue(isNaN(result.tenkan_sen[0]))
        assert.isTrue(isNaN(result.kijun_sen[0]))
        assert.isTrue(isNaN(result.senkou_span_a[0]))
        assert.isTrue(isNaN(result.senkou_span_b[0]))
    })

    test('should calculate correct Tenkan-sen values', ({ assert }) => {
        const testData = generateTestData(100)

        const lhocvd = lowHighOpenCloseVolumeDateToArray(testData)
        const result = ichimoku(testData)

        // First 8 values should be NaN (9-period calculation)
        for (let i = 0; i < 8; i++) {
            assert.isTrue(isNaN(result.tenkan_sen[i]))
        }

        const expectedMax = Math.max(...lhocvd.highs.slice(0, 9));
        const expectedMin = Math.min(...lhocvd.lows.slice(0, 9));
        const expectedTenkan = (expectedMax + expectedMin) / 2;

        assert.approximately(result.tenkan_sen[8], expectedTenkan, 0.01);
    })

    test('should handle empty data', ({ assert }) => {
        assert.rejects(() => ichimoku([]))
    })

    test('should verify Senkou Span displacement', ({ assert }) => {
        const testData = generateTestData(60)
        const result = ichimoku(testData)

        // First 26 values should be NaN due to displacement
        for (let i = 0; i < 26; i++) {
            assert.isTrue(isNaN(result.senkou_span_a[i]))
            assert.isTrue(isNaN(result.senkou_span_b[i]))
        }
    })

    test('should calculate correct Chikou Span', ({ assert }) => {
        const testData = generateTestData(60)
        const result = ichimoku(testData)

        // Last 26 values should be NaN
        for (let i = testData.length - 26; i < testData.length; i++) {
            assert.isTrue(isNaN(result.chikou_span[i]))
        }

        // Other values should match closing prices shifted backwards
        for (let i = 0; i < testData.length - 26; i++) {
            assert.equal(result.chikou_span[i], testData[i + 26].close)
        }
    })
})
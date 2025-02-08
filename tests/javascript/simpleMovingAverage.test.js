import { test } from '@japa/runner'
import wasmModule from '../../dist/index.js'

test.group('SimpleMovingAverage', async (group) => {

    let SimpleMovingAverage
    group.setup(async () => {
        const module = await wasmModule
        SimpleMovingAverage = module.SimpleMovingAverage
    })

    test('should return NaN for SMA when prices are less than period', async ({ assert }) => {
        const sma = new SimpleMovingAverage([1.0, 2.0, 3.0])
        const result = sma.period(5);
        const resultArray = Array.from(result);
        assert.deepEqual(resultArray, []);
    })

    test('should calculate SMA correctly', async ({ assert }) => {
        const sma = new SimpleMovingAverage([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        const result = sma.period(3)
        assert.equal(result[2], 2.0)
        assert.equal(result[3], 3.0)
        assert.equal(result[4], 4.0)
        assert.equal(result[5], 5.0)
    })

    test('should return empty array if period is greater than prices length', async ({ assert }) => {
        const sma = new SimpleMovingAverage([1.0])
        const result = sma.period(2)
        assert.equal(result.length, 0)
    })
})

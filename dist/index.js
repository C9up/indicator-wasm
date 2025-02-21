import * as indicator from './node/technical_indicators_wasm.js'

export class Indicator {

    #data = []

    constructor(data) {
        this.#data = data
    }

    ichimoku(tenkan, kijun, senkou) {
        return ichimoku(this.#data, tenkan, kijun, senkou)
    }

    directionalMovementIndex(period) {
        return directionalMovementIndex(this.#data, period)
    }

    relativeStrengthIndex(period) {
        return relativeStrengthIndex(this.#data, period)
    }

    extractImportantLevels() {
        return extractImportantLevels(this.#data)
    }

    bollingerBands(period, multiplier) {
        return bollingerBands(this.#data, period, multiplier)
    }

    simpleMovingAverage(period) {
        return simpleMovingAverage(this.#data, period)
    }

    lowHighOpenCloseVolumeDateToArray(data){
        return lowHighOpenCloseVolumeDateToArray(data)
    }

    entryExitSignals(sma_period, ema_period, atr_period, threshold) {
        return entryExitSignals(this.#data,sma_period, ema_period, atr_period, threshold)
    }

    stochasticMomentumIndex(period_k, period_d) {
        return stochasticMomentumIndex(this.#data, period_k, period_d)
    }

    stochasticOscillator(period) {
        return stochasticOscillator(this.#data, period)
    }

    trendsMeter(period) {
        return trendsMeter(this.#data, period)
    }

    exponentialMovingAverage(period) {
        return exponentialMovingAverage(this.#data, period)
    }

    parabolicSar(start, increment, max_value) {
        return indicator.parabolic_sar(this.#data, start, increment, max_value)
    }

    renkoChart(brickSize) {
        return renkoChart(this.#data, brickSize)
    }

    kagiChart(reversalAmount) {
        return kagiChart(this.#data, reversalAmount)
    }
}

export function ichimoku(data, tenkan, kijun, senkou) {
    return indicator.ichimoku(data, tenkan, kijun, senkou)
}

export function directionalMovementIndex(data, period) {
    if (period <= 0) {
        throw new Error('Period must be greater than 0.');
    }
    return Array.from(indicator.directional_movement_index(data, period))
}

export function relativeStrengthIndex(data, period) {
    if (period <= 0) {
        throw new Error('Period must be greater than 0.');
    }
    return indicator.relative_strength_index(data, period)
}

export function extractImportantLevels(data) {
    return indicator.extract_important_levels(data)
}

export function bollingerBands(data, period, multiplier) {
    if (period <= 0) {
        throw new Error('Period must be greater than 0.');
    }
    return indicator.bollinger_bands(data, period, multiplier)
}

export function simpleMovingAverage(data, period) {
    return Array.from(indicator.simple_moving_average(new Float64Array(data), period))
}

export function lowHighOpenCloseVolumeDateToArray(data){
    return indicator.lowHighOpenCloseVolumeDateToArray(data)
}

export function entryExitSignals(data, sma_period, ema_period, atr_period, threshold) {
    return indicator.entry_exit_signals(data,sma_period, ema_period, atr_period, threshold)
}

export function stochasticMomentumIndex(data, period_k, period_d) {
    return Array.from(indicator.stochastic_momentum_index(data, period_k, period_d))
}

export function stochasticOscillator(data, period) {
    return indicator.stochastic_oscillator(data, period)
}

export function trendsMeter(data, period) {
    return indicator.trends_meter(data, period)
}

export function exponentialMovingAverage(data, period) {
    if (period <= 0) {
        throw new Error('Period must be greater than 0.');
    }
    return indicator.exponential_moving_average(data, period)
}

export function renkoChart(data, brickSize) {
    return indicator.renko_chart(data, brickSize)
}

export function kagiChart(data, reversalAmount) {
    return indicator.kagi_chart(data, reversalAmount)
}

export function parabolicSar(data, start, increment, max_value) {
    return Array.from(indicator.parabolic_sar(data, start, increment, max_value))
}
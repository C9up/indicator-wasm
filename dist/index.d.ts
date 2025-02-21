export type Data = {
    low: number
    high: number
    open: number
    close: number
    volume: number
    date: string | Date
}

export type DataSegmented = {
    low: number[]
    high: number[]
    open: number[]
    close: number[]
    volume: number[]
    date: string
}

export type IchimokuResponse = {
    tenkan_sen: number[]
    kijun_sen: number[]
    senkou_span_a: number[]
    senkou_span_b: number[]
    chikou_span: number[]
}

export type BollingerBandResult = {
    middle: number[]
    upper: number[]
    lower: number[]
}

export declare class Indicator {
    constructor(data: Data[])
    ichimoku(tenkan: number, kijun: number, senkou: number): IchimokuResponse;
    directionalMovementIndex(period: number): Float64Array;
    relativeStrengthIndex(key: keyof Data, period: number): Float64Array;
    extractImportantLevels(key: keyof Data): any;
    bollingerBands(period: number, multiplier: number): BollingerBandResult;
    simpleMovingAverage(key: keyof Data, period: number): Float64Array;
    lowHighOpenCloseVolumeDateToArray(data: Data[]): DataSegmented;
    entryExitSignals(key: keyof Data, sma_period: number, ema_period: number, atr_period: number, threshold: number): any;
    stochasticMomentumIndex(period_k: number, period_d: number): void;
    stochasticOscillator(period: number): Float64Array;
    trendsMeter(period: number): Float64Array;
    exponentialMovingAverage(period: number): Float64Array;
    renkoChart(brickSize: number): Float64Array;
    kagiChart(reversalAmount: number): any;
}
export declare function ichimoku(data: Data[], tenkan: number, kijun: number, senkou: number): IchimokuResponse;
export declare function directionalMovementIndex(data: Data[], period: number): Float64Array;
export declare function relativeStrengthIndex(data: Data[], period: number): Float64Array;
export declare function extractImportantLevels(data: Data[]): any;
export declare function bollingerBands(data: Data[], period: number, multiplier: number): BollingerBandResult;
export declare function simpleMovingAverage(data: Data[], period: number): Float64Array;
export declare function lowHighOpenCloseVolumeDateToArray(data: Data[]): DataSegmented;
export declare function entryExitSignals(data: Data[], sma_period: number, ema_period: number, atr_period: number, threshold: number): any;
export declare function stochasticMomentumIndex(data: Data[], period_k: number, period_d: number): void;
export declare function stochasticOscillator(data: Data[], period: number): Float64Array;
export declare function trendsMeter(data: Data[], period: number): Float64Array;
export declare function exponentialMovingAverage(data: Data[], period: number): Float64Array;
export declare function renkoChart(data: Data[], brickSize: number): Float64Array;
export declare function kagiChart(data: Data[], reversalAmount: number): any;

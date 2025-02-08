
let wasmModule;

if (typeof window !== 'undefined') {
    wasmModule = import('./pkg/web/technical_indicators_wasm.js');
} else {
    wasmModule = import('./pkg/node/technical_indicators_wasm.js');
}

export default wasmModule;

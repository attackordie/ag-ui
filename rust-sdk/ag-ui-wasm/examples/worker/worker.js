import init, { fetch as wasmFetch } from './pkg/ag_ui_worker_example.js';
import wasmModule from './pkg/ag_ui_worker_example_bg.wasm';

// Initialize WASM module
let wasmInitialized = false;
let wasmInitPromise = null;

async function initWasm() {
  if (wasmInitialized) return;
  if (wasmInitPromise) return wasmInitPromise;
  
  // Pass the WASM module directly to avoid URL issues
  wasmInitPromise = init(wasmModule);
  await wasmInitPromise;
  wasmInitialized = true;
}

export default {
  async fetch(request, env, ctx) {
    // Initialize WASM on first request
    await initWasm();
    
    // Call the Rust fetch handler
    return wasmFetch(request);
  }
}; 
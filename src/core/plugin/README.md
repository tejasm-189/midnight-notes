# src/core/plugin/ — WASM Plugin Host

## Purpose
WebAssembly plugin runtime for user-written note processing plugins. Feature-gated behind `plugins` flag.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root |
| `host.rs` | ✅ done | WASM runtime (wasmtime), PluginManager, WasmPlugin loader |
| `api.rs` | ✅ done | PluginInput/PluginOutput types, output validation |

## Progress
- [x] Module structure
- [x] WASM plugin host with wasmtime runtime
- [x] Plugin manager: load .wasm from directory
- [x] Plugin API types (input, output, validation)
- [x] Feature-gated behind `plugins` flag

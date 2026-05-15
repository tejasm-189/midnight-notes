# Tests — src/core/plugin/

## Unit Tests (3 tests)
| File | Test | What it verifies |
|------|------|-----------------|
| `host.rs` | `test_empty_plugin_manager` | New manager has no plugins |
| `host.rs` | `test_load_nonexistent_directory` | Loading from nonexistent dir returns empty |
| `host.rs` | `test_process_all_empty` | Processing with no plugins returns empty |

## Running
```bash
cargo test core::plugin  # run plugin tests only
cargo test --features plugins  # with wasmtime enabled
```

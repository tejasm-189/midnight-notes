# Tests — src/core/plugin/

## Unit Tests (3 tests)
| File | Test | What it verifies |
|------|------|-----------------|
| `host.rs` | `new_plugin_manager_has_no_plugins` | New manager has no plugins |
| `host.rs` | `loading_from_nonexistent_directory_returns_empty` | Loading from nonexistent dir returns empty |
| `host.rs` | `processing_with_no_plugins_returns_empty` | Processing with no plugins returns empty |

## Running
```bash
cargo test core::plugin  # run plugin tests only
cargo test --features plugins  # with wasmtime enabled
```

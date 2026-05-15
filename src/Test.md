# Tests — src/

## Integration Tests
The following modules support end-to-end tests that span across multiple layers:

| Module | Test coverage |
|--------|--------------|
| `core/` | 62 unit tests across 9 modules |
| `crypto/` | 12 unit tests |
| `storage/` | 9 unit tests |
| `ui/editor/` | 13 unit tests |
| `tests/` | 11 integration tests + 2 doc-tests |

## Running
```bash
cargo test                    # all unit + integration + doc tests
cargo test --lib              # unit tests only (lib crate)
cargo test --test integration  # integration tests only
```

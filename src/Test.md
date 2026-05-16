# Tests — src/

## Integration Tests
Settings is now inline in workspace (no longer separate screen).
The following modules support end-to-end tests that span across multiple layers:

| Module | Test coverage |
|--------|--------------|
| `core/` | 62 unit tests across 9 modules |
| `crypto/` | 12 unit tests |
| `storage/` | 9 unit tests |
| `ui/editor/` | 17 unit tests (vim 8 + slash 5 + spreadsheet 2 + kanban 2) |
| `ui/` | 13 unit + `sidebar/` 0 test + 3 manual workflows (enhanced templates, seamless editors) |
| `tests/` | 11 integration tests + 2 doc-tests |

## Running
```bash
cargo test                    # all unit + integration + doc tests
cargo test --lib              # unit tests only (lib crate)
cargo test --test integration  # integration tests only
```




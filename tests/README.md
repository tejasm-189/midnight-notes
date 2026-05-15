# tests/ — Integration Tests

## Files

| File | Status | Description |
|------|--------|-------------|
| `integration.rs` | ✅ done | 9 integration tests: note lifecycle, tags, backlinks, search, history, export, markdown |

## Test types
- **Unit tests**: 97 — inside each module (`#[cfg(test)]`)
- **Integration tests**: 9 — in `tests/integration.rs`
- **Doc tests**: 2 — in API documentation comments
- **Total**: 108 tests

## Running
```bash
cargo test                    # all tests
cargo test --test integration # integration only
cargo test test_note_lifecycle # specific test
```

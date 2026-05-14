# src/core/ — Business Logic

## Purpose
All application business logic. No Dioxus imports allowed. Pure Rust services for notes CRUD, tag management, search, backlinks, version history, and WASM plugin hosting.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root, error types |
| `note.rs` | ✅ done | Note CRUD service (create, read, update, delete, trash, pin, archive, FTS5 search) |
| `tag.rs` | 📋 planned | Tag model + tree operations |
| `search.rs` | 📋 planned | Tag model + tree operations |
| `backlinks.rs` | 📋 planned | Bidirectional link resolver |
| `history.rs` | 📋 planned | Version snapshots + diff |
| `export.rs` | 📋 planned | Encrypted export logic |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `plugin/` | 📋 planned | WASM plugin host (wasmtime) |

## Progress
- [x] Note CRUD (create, read, update, delete, trash, toggle_pin, toggle_archive)
- [x] FTS5 search
- [x] List active (pinned first) + list trashed
- [ ] Tag model + tree
- [ ] Backlink resolution
- [ ] Version history
- [ ] Encrypted exports
- [x] Unit tests (9 tests)

## Notes
- Must not import Dioxus or Tauri types
- All I/O goes through `storage/` layer

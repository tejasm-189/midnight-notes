# src/core/ — Business Logic

## Purpose
All application business logic. No Dioxus imports allowed. Pure Rust services for notes CRUD, tag management, search, backlinks, version history, and WASM plugin hosting.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root, error types |
| `note.rs` | ✅ done | Note CRUD service (create, read, update, delete, trash, pin, archive, FTS5 search) |
| `tag.rs` | ✅ done | Tag CRUD + tree operations + note-tag assignment |
| `markdown.rs` | ✅ done | Markdown renderer (pulldown-cmark), plain text extractor, link parser |
| `search.rs` | ✅ done | FTS5 search with tag:/has:todo/path: filters + smart views |
| `backlinks.rs` | ✅ done | [[Note]] backlink extraction, resolution, linked mentions |
| `history.rs` | ✅ done | Version snapshots: list, get, restore, diff between versions |
| `export.rs` | ✅ done | Encrypted zip export/import with XChaCha20-Poly1305 |
| `watcher.rs` | ✅ done | File system watcher via notify (create/modify/delete events) |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `plugin/` | ✅ done | WASM plugin host (wasmtime), manager, API types |

## Progress
- [x] Note CRUD with XChaCha20-Poly1305 encryption
- [x] FTS5 search with tag:/has:todo/path: filters
- [x] Tag CRUD + tree (nested parent/child)
- [x] Note-tag assignment
- [x] [[Wiki-link]] backlink extraction + resolution
- [x] Linked mentions + outgoing links
- [x] Smart views (saved FTS5 queries)
- [x] Version history (list, get, restore, diff)
- [x] Encrypted zip export/import with salt in metadata
- [x] WASM plugin host + manager + file watcher
- [x] Unit tests: 97 unit + 9 integration + 2 doc = 108
- [x] Encryption: Argon2id key derivation, XChaCha20-Poly1305 encrypt/decrypt

## Notes
- Must not import Dioxus or Tauri types
- All I/O goes through `storage/` layer

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
| `history.rs` | 📋 planned | Version snapshots + diff |
| `export.rs` | 📋 planned | Encrypted export logic |

## Progress
- [x] Note CRUD (create, read, update, delete, trash, toggle_pin, toggle_archive)
- [x] FTS5 search
- [x] Tag CRUD + tree (nested parent/child)
- [x] Note-tag assignment
- [x] [[Wiki-link]] backlink extraction + resolution
- [x] Linked mentions + outgoing links
- [x] Smart views (saved FTS5 queries)
- [ ] Version history
- [ ] Encrypted exports
- [x] Unit tests (33 tests)

## Notes
- Must not import Dioxus or Tauri types
- All I/O goes through `storage/` layer

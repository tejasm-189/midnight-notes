# src/storage/ — Data Persistence

## Purpose
SQLite database layer with application-layer XChaCha20-Poly1305 encryption. Handles connections, migrations, and row-to-struct mappings. No business logic.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root, re-exports |
| `database.rs` | ✅ done | SQLite + WAL mode, encryption key storage, in-memory support |
| `migrations.rs` | ✅ done | Schema (notes, tags, note_tags, backlinks, note_history, FTS5, triggers) |
| `models.rs` | ✅ done | Row structs (Note, Tag, Backlink, NoteHistory, SearchResult) |
| `attachments.rs` | ✅ done | Blob file management (.attachments/ directory) |

## Progress
- [x] Database connection + WAL mode
- [x] Schema (11 migrations: tables, FTS5, triggers, meta)
- [x] Model mappings
- [x] Attachment handling
- [x] Encryption key management on Database struct
- [x] Unit tests

# src/storage/ — Data Persistence

## Purpose
SQLCipher-encrypted SQLite database layer. Handles connections, migrations, and row-to-struct mappings. No business logic.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root, re-exports |
| `database.rs` | ✅ done | SQLite connection (WAL mode, in-memory support) |
| `migrations.rs` | ✅ done | Schema migrations (notes, tags, FTS5, triggers) |
| `models.rs` | ✅ done | Row ↔ struct mappings (Note, Tag, Backlink, etc.) |
| `attachments.rs` | ✅ done | Blob file management |

## Progress
- [x] Database connection + WAL mode
- [x] Schema migrations (v1-v4)
- [x] Model mappings
- [x] Attachment handling
- [x] Unit tests (6 tests)

## Notes
- Entire DB file encrypted via SQLCipher (XChaCha20-Poly1305)
- Key derived from master password via Argon2id (done in `crypto/`)
- FTS5 for full-text search

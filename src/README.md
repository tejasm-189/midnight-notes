# src/ — Source Code

## Purpose
All application source code. Organized by layer: UI (Dioxus components), Core (business logic), Storage (SQLite persistence), and Crypto (encryption utilities).

## Files

| File | Status | Description |
|------|--------|-------------|
| `main.rs` | ✅ done | Dioxus Desktop entry point |
| `lib.rs` | ✅ done | App launch function |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `ui/` | ✅ done | Dioxus frontend: workspace, settings (with sidebar), vault lock, editors, sidebar |
| `core/` | ✅ done | Business logic: notes, tags, search, backlinks, history, export, markdown, plugins, watcher |
| `storage/` | ✅ done | SQLite persistence: database, migrations, models, attachments |
| `crypto/` | ✅ done | Encryption: Argon2id, XChaCha20-Poly1305, AES-256-GCM |

## Progress
- [x] Project scaffold
- [x] Module structure
- [x] 114 tests (101 unit + 11 integration + 2 doc)

## Notes
- No file in `src/` should import Dioxus directly (UI layer only)

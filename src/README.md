# src/ — Source Code

## Purpose
All application source code. Organized by layer: UI (Dioxus components), Core (business logic), Storage (SQLCipher persistence), and Crypto (encryption utilities).

## Files

| File | Status | Description |
|------|--------|-------------|
| `main.rs` | 📋 planned | Tauri entry point |
| `lib.rs` | 📋 planned | Shared library root, re-exports |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `ui/` | 📋 planned | Dioxus frontend components |
| `core/` | 📋 planned | Business logic services |
| `storage/` | 📋 planned | SQLCipher data persistence |
| `crypto/` | 📋 planned | Encryption utilities |

## Progress
- [ ] Project scaffold
- [ ] Module structure
- [ ] Tests

## Notes
- No file in `src/` should import Dioxus directly (UI layer only)

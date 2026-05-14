# src/ui/ — Dioxus Frontend Components

## Purpose
All Dioxus components for the user interface. Organized by feature/screen. No business logic — components call core services via Tauri IPC commands.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root, re-exports |
| `app.rs` | ✅ done | Root component with screen routing (VaultLock / Workspace) |
| `vault_lock.rs` | ✅ done | Vault unlock screen (biometric + master password) |
| `workspace.rs` | ✅ done | Main workspace (sidebar, note list, editor with mode switcher) |
| `viewer.rs` | 📋 planned | Note viewer/reader |
| `search.rs` | 📋 planned | Search bar + results display |
| `settings.rs` | 📋 planned | Settings panel |
| `theme.rs` | 📋 planned | Theme switcher logic |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `editor/` | 📋 planned | Multi-mode editor components |
| `sidebar/` | 📋 planned | Navigation sidebar components |

## Progress
- [x] Root component with routing (VaultLock ↔ Workspace)
- [x] Vault lock screen (matches `docs/Design/vault-lock.html`)
- [x] Main workspace shell (sidebar, note list, editor)
- [ ] Note viewer
- [ ] Search/Smart Views screen
- [ ] Settings screen
- [ ] Theme system

## Notes
- Follow the Midnight design system in `DESIGN.md`
- Reference HTML mockups in `docs/Design/`

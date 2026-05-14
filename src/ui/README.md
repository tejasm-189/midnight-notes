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
| `settings.rs` | ✅ done | Settings panel with theme switcher (Midnight/Dark/Light) |
| `theme.rs` | ✅ done | Theme provider system, CSS custom properties, 3 themes |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `editor/` | ✅ done | Multi-mode editor components |
| `sidebar/` | 📋 planned | Navigation sidebar components |

## Progress
- [x] Root component with routing (VaultLock ↔ Workspace ↔ Settings)
- [x] Vault lock screen
- [x] Main workspace shell
- [x] Settings screen with theme selector
- [x] Theme system: 3 themes (Midnight/Dark/Light) via CSS custom properties
- [ ] Search/Smart Views screen

## Notes
- Follow the Midnight design system in `DESIGN.md`
- Reference HTML mockups in `docs/Design/`

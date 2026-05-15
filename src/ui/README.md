# src/ui/ — Dioxus Frontend Components

## Purpose
All Dioxus components for the user interface. Organized by feature/screen. No business logic — components call core services via Tauri IPC commands.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root, re-exports |
| `app.rs` | ✅ done | Root component with screen routing (VaultLock / Workspace) — Settings now inline in workspace |
| `vault_lock.rs` | ✅ done | Vault unlock screen (biometric + master password) |
| `workspace.rs` | ✅ done | Main workspace: sidebar, note list with tag chips + context menu, editor (ProseEditor/CodeEditor/Vim modes), auto-save, calendar toggle |
| `viewer.rs` | ✅ done | Read-only note viewer with markdown rendering |
| `smart_view.rs` | ✅ done | Smart Views screen: FTS5 search, results grid, linked graph |
| `settings.rs` | ✅ done | Settings screen with sidebar, theme switcher (Midnight/Dark/Light) |
| `theme.rs` | ✅ done | Theme provider system, ThemeColors struct, 3 themes |

## Sub-folders

| Folder | Status | Description |
|--------|--------|-------------|
| `editor/` | ✅ done | Prose/Code/Vim editors, slash commands |
| `sidebar/` | ✅ done | Backlinks panel, tag tree, calendar, smart views list, shared SidebarItem component |

## Progress
- [x] Root component with routing (VaultLock ↔ Workspace ↔ Settings)
- [x] Vault lock screen with biometric + password
- [x] Workspace: sidebar, note list, editor, auto-save, tags, history, context menu
- [x] Smart Views: FTS5 search, results grid, linked graph panel
- [x] Settings: theme selector (Midnight/Dark/Light)
- [x] Theme system: 3 themes, ThemeColors struct for Rust-level theme
- [x] Calendar sidebar: date picker month grid, daily notes
- [x] Context menu on note cards (Pin, Archive, Delete via ⋮ button)
- [x] Clickable tag chips on note cards (click to filter by tag)
- [x] Date format: `Mon 15 May 2026 01:03 PM`
- [x] Prose/Code/Vim mode switching in editor header
- [x] Backlinks panel, tag tree, smart views list in sidebar

## Notes
- Follow the Midnight design system in `DESIGN.md`
- Reference HTML mockups in `docs/Design/`

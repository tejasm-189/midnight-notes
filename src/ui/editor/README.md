# src/ui/editor/ — Multi-Mode Editor

## Purpose
Editor components for Prose, Code, and Vim modes, plus slash command palette.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root |
| `prose.rs` | ✅ done | Markdown editing with live preview (theme-aware, border divider) |
| `code.rs` | ✅ done | Code editing with monospace font, theme-aware background |
| `vim.rs` | ✅ done | Vim mode state machine (normal/insert/visual/command) |
| `spreadsheet.rs` | ✅ done | Spreadsheet editor (JSON persistence) |
| `kanban.rs` | ✅ done | Kanban board editor (JSON persistence) |
| `slash.rs` | ✅ done | Slash command palette (12 commands) |

## Progress
- [x] Module structure
- [x] Prose editor with split textarea/preview
- [x] Code editor with monospace font
- [x] Vim mode state machine + status bar
- [x] Spreadsheet editor with dynamic grid
- [x] Kanban board editor with drag-ready columns
- [x] Slash command menu (table, code, image, math, todo, etc.)

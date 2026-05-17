# src/ui/editor/ — Multi-Mode Editor

## Purpose
Editor components for Prose, Code, and Vim modes, plus slash command palette.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root |
| `prose.rs` | ✅ done | Live preview-first markdown editor with cleaner dock-style canvas and starter checklist/code preview |
| `richtext.rs` | ✅ done | WYSIWYG editor (HTML editing + live markdown conversion and seamless borderless theme layout). Features integrated Ctrl+Enter breakout logic, continuous Shift+Enter code-writing, and dynamic Code Block Copy overlay buttons. |
| `code.rs` | ✅ done | Code editor with line-number gutter and integrated checklist side panel |
| `vim.rs` | ✅ done | Vim mode state machine (normal/insert/visual/command) |
| `spreadsheet.rs` | ✅ done | Spreadsheet editor (JSON persistence) |
| `kanban.rs` | ✅ done | Kanban board editor (JSON persistence) |
| `slash.rs` | ✅ done | Slash command palette (12 commands) |
| `format_menu.rs` | ✅ done | Consolidated context menu with rich text formatting command support and unified editor mode selection (RichText, Code, Vim, Sheet, Board) |

## Progress
- [x] Module structure
- [x] Prose editor with preview-first live rendering
- [x] WYSIWYG Rich text editor (no raw markdown or HTML inside, toolbar and right-click actions)
- [x] Rich text editor: Ctrl+Enter code breakout to paragraph below
- [x] Rich text editor: Shift+Enter or Enter inside code block inserts a clean newline character
- [x] Rich text editor: Premium Copy overlay button on code blocks appearing on hover
- [x] Code editor with monospace font
- [x] Vim mode state machine + status bar
- [x] Spreadsheet editor with dynamic grid
- [x] Kanban board editor with drag-ready columns
- [x] Slash command menu (table, code, image, math, todo, etc.)

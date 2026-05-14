# src/ui/editor/ — Multi-Mode Editor

## Purpose
Editor components for Prose, Code, and Vim modes, plus slash command palette.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root |
| `prose.rs` | ✅ done | Markdown editing with live preview (pulldown-cmark) |
| `code.rs` | ✅ done | Code editing with monospace editor |
| `vim.rs` | ✅ done | Vim mode state machine (normal/insert/visual/command) |
| `slash.rs` | ✅ done | Slash command palette (12 commands) |

## Progress
- [x] Module structure
- [x] Prose editor with split textarea/preview
- [x] Code editor with monospace font
- [x] Vim mode state machine + status bar
- [x] Slash command menu (table, code, image, math, todo, etc.)

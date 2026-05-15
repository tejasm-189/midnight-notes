# Tests — src/ui/

## Unit Tests
| File | Tests | Description |
|------|-------|-------------|
| `editor/vim.rs` | 8 | Vim mode state machine |
| `editor/slash.rs` | 5 | Slash command filtering |
| `editor/spreadsheet.rs` | 2 | Spreadsheet logic |
| `editor/kanban.rs` | 2 | Kanban logic |

## Integration / Manual Testing
UI components are primarily tested via visual inspection (Dioxus doesn't have a built-in test harness). Key flows to verify:

| Screen | What to test |
|--------|-------------|
| Vault Lock | Password input, biometric button, unlock flow |
| Workspace | Note list, Prose/Code/Sheet/Board editors, Vim mode, auto-save, pin, archive, trash, context menu (⋮), tag chip filtering |
| Settings | Inline in workspace, sidebar nav, theme switching (Light/Dark/Midnight), About section |
| Smart Views | Search query, results, linked graph |
| Calendar | Month navigation, date selection, daily note creation, date format |

## Running
```bash
cargo test ui::editor          # run editor tests only
cargo run                      # manual UI verification
```

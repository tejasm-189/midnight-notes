# Tests — src/ui/

## Unit Tests
| File | Tests | Description |
|------|-------|-------------|
| `editor/vim.rs` | 8 | Vim mode state machine |
| `editor/slash.rs` | 5 | Slash command filtering |
| `editor/spreadsheet.rs` | 2 | Spreadsheet logic |
| `editor/kanban.rs` | 2 | Kanban logic |

## Integration / Manual Testing
UI components are tested via visual inspection. Key flows to verify:

| Screen | What to test |
|--------|-------------|
| Vault Lock | Password input, biometric button, unlock flow |
| Workspace | Note list, Prose/Code (seamless Apple-inspired)/Sheet/Board editors, Focus Mode (fullscreen editor), Vim mode, auto-save, pin, archive, trash, context menu (⋮), tag chip filtering, persistent tag tree |
| Settings | Sidebar nav, encryption (Key Mgmt/Argon2), theme cards, sync paths, biometric toggle |
| Smart Views | Search query, results, linked graph |
| Calendar | Month navigation, date selection, daily note creation (enhanced template), view transition from Smart Views, date format |

## Running
```bash
cargo test ui::editor          # run editor tests only
cargo run                      # manual UI verification
```


# Tests — src/ui/editor/

## vim.rs (8 tests)
| Test | What it verifies |
|------|-----------------|
| `test_normal_to_insert` | `i`/`a` switches to Insert mode |
| `test_insert_to_normal` | Escape switches to Normal mode |
| `test_insert_passthrough` | Character keys pass through in Insert |
| `test_normal_to_visual` | `v` switches to Visual mode |
| `test_visual_to_normal` | Escape returns to Normal |
| `test_normal_to_command` | `:` switches to Command mode |
| `test_command_enter` | Enter returns to Normal |
| `test_mode_labels` | Each mode has correct label |

## slash.rs (5 tests)
| Test | What it verifies |
|------|-----------------|
| `test_filter_commands_empty` | Empty query returns all commands |
| `test_filter_commands_table` | `tab` matches "table" |
| `test_filter_commands_code` | `code` matches "Code Block" |
| `test_filter_commands_no_match` | Unknown query returns empty |
| `test_find_exact_command` | Exact trigger match works |

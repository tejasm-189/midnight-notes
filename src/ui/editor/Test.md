# Tests — src/ui/editor/

## prose.rs / code.rs
No unit tests (UI components). Updated to use theme colors, removed duplicate title inputs. Prose editor now uses a split-view layout to ensure content is always editable.

## spreadsheet.rs / kanban.rs
Now use theme colors, full-height layout.
No unit tests added.

# Tests — src/ui/editor/

## vim.rs (8 tests)
| Test | What it verifies |
|------|-----------------|
| `pressing_i_switches_normal_to_insert_mode` | `i`/`a` switches to Insert mode |
| `pressing_escape_switches_insert_to_normal_mode` | Escape switches to Normal mode |
| `character_keys_pass_through_in_insert_mode` | Character keys pass through in Insert |
| `pressing_v_switches_to_visual_mode` | `v` switches to Visual mode |
| `pressing_escape_returns_to_normal_mode` | Escape returns to Normal |
| `pressing_colon_switches_to_command_mode` | `:` switches to Command mode |
| `pressing_enter_in_command_mode_returns_to_normal` | Enter returns to Normal |
| `each_vim_mode_has_correct_label` | Each mode has correct label |

## spreadsheet.rs (2 tests)
| Test | What it verifies |
|------|-----------------|
| `adding_row_increases_row_count` | Clicking add row updates JSON state |
| `editing_cell_updates_json_payload` | Input changes are reflected in note content |

## kanban.rs (2 tests)
| Test | What it verifies |
|------|-----------------|
| `adding_card_to_column_updates_tasks` | New task is added to the correct column list |
| `renaming_column_updates_title` | Column title change persists in JSON |

## slash.rs (5 tests)
| Test | What it verifies |
|------|-----------------|
| `empty_query_returns_all_commands` | Empty query returns all commands |
| `tab_query_matches_table_command` | `tab` matches "table" |
| `code_query_matches_code_block_command` | `code` matches "Code Block" |
| `unknown_query_returns_empty_results` | Unknown query returns empty |
| `exact_trigger_match_finds_command` | Exact trigger match works |

## format_menu.rs
No unit tests (UI component). Visual verification: context menu opens, formatting options (bold, italic) insert correct markdown.

# Tests — src/ui/editor/

## prose.rs / richtext.rs / code.rs
No unit tests (UI components).
- Prose editor uses a clean dock-style live preview canvas.
- Richtext editor renders a fully custom, borderless WYSIWYG workspace with a dynamic toolbar. It translates HTML elements directly to markdown when the note is edited. Verify formatting via toolbar commands (Bold, Italic, Underline, Headings, Lists, etc.) works seamlessly without any visible markdown characters.
- Verify Code Block Breakouts and Shortcuts in Richtext:
  1. Create or place cursor in a Code Block (`<pre>`).
  2. Pressing `Enter` or `Shift+Enter` should always insert a clean newline (`\n`) and keep cursor focus in the code editor.
  3. Pressing `Ctrl+Enter` (or `Cmd+Enter` on Mac) should breakout of the code block, trim any trailing newlines from the block, and create a fresh paragraph below it.
  4. Hovering over a Code Block should reveal a sleek, premium "Copy" button. Clicking it should copy the code block text and trigger a green "Copied!" success state for 2 seconds.
- Code editor includes line-number gutter and an integration checklist side panel.

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
No unit tests (UI component). Visual verification: right-clicking the editor opens the consolidated context menu. Users can switch the note's active editor mode (RichText, Code Editor, Vim Editor, Spreadsheet, Kanban Board) directly from the menu. In Vim/Code modes, clicking an action (Bold, Heading, Table, etc.) inserts markdown. In RichText mode, it applies formatting dynamically to the selection or editor cursor instantly.

# Tests — src/core/

## note.rs (12 tests)
| Test | What it verifies |
|------|-----------------|
| `test_create_note` | Create note with title + content |
| `test_get_note` | Get note by ID |
| `test_get_nonexistent` | Nonexistent ID returns None |
| `test_update_note` | Update title and content |
| `test_trash_and_list` | Trash removes from active, shows in trashed |
| `test_delete_note` | Permanent delete removes note |
| `test_toggle_pin` | Toggle pin toggles the pinned flag |
| `test_list_active_ordering` | Pinned notes appear first |
| `test_search` | FTS5 search finds matching notes |
| `test_list_archived` | Archived notes list |
| `test_restore_from_trash` | Trashed → restored → appears in active |
| `test_delete_permanently` | Permanent delete removes note completely |

## tag.rs (12 tests)
| Test | What it verifies |
|------|-----------------|
| `test_create_and_get` | Create tag, get by ID |
| `test_create_with_parent` | Tag with parent_id creates hierarchy |
| `test_get_by_name` | Get tag by name |
| `test_update` | Update name / color |
| `test_delete` | Delete removes tag and note_tags |
| `test_list_roots` | Root tags (no parent) listed |
| `test_get_children` | Children of a parent tag |
| `test_assign_and_get_tags_for_note` | Assign tag to note, list tags for note |
| `test_remove_from_note` | Remove tag from note |
| `test_empty_name_rejected` | Empty tag name returns error |
| `test_get_all` | List all tags |

## backlinks.rs (9 tests)
| Test | What it verifies |
|------|-----------------|
| `test_extract_links` | `[[Title]]` extracted from content |
| `test_extract_links_no_matches` | No links → empty list |
| `test_extract_links_empty_content` | Empty string → empty list |
| `test_extract_links_trims_whitespace` | `[[  Title  ]]` → `Title` |
| `test_refresh_and_get_mentions` | Refresh backlinks, get linked mentions |
| `test_outgoing_links` | Notes this note links to |
| `test_refresh_replaces_old_links` | Refresh replaces old backlinks |
| `test_resolve_title_exact` | Exact title match resolves to ID |
| `test_resolve_title_nonexistent` | Nonexistent title returns None |

## search.rs (8 tests)
| Test | What it verifies |
|------|-----------------|
| `test_parse_query_plain` | Plain text query returns no filters |
| `test_parse_query_with_tag` | `tag:work` extracts tag filter |
| `test_parse_query_has_todo` | `has:todo` extracts filter |
| `test_parse_query_path` | `path:docs/` extracts path filter |
| `test_parse_query_all_filters` | Combined filters all extracted |
| `test_search_basic` | FTS5 search finds matching notes |
| `test_smart_view_crud` | Save/list/delete smart views |
| `test_smart_view_execute` | Execute saved smart view |

## history.rs (6 tests)
| Test | What it verifies |
|------|-----------------|
| `test_list_history_empty` | New note has no history |
| `test_list_history_after_update` | Update creates a snapshot |
| `test_get_snapshot` | Get snapshot by history ID |
| `test_restore_snapshot` | Restore reverts content |
| `test_diff_snapshots` | Diff between two snapshots |
| `test_get_nonexistent` | Nonexistent snapshot returns None |

## export.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `test_export_import_roundtrip` | Export then import preserves content |
| `test_wrong_password_fails` | Wrong password fails import |
| `test_export_empty_list` | Export with no notes still works |

## markdown.rs (11 tests)
| Test | What it verifies |
|------|-----------------|
| `test_render_paragraph` | Bold markdown renders as `<strong>` |
| `test_render_table` | Tables render with `<th>`/`<td>` |
| `test_render_tasklist` | Task lists render with `checked` |
| `test_plain_text_summary` | Text extraction truncates at limit |
| `test_plain_text_summary_short` | Short text doesn't truncate |
| `test_plain_text_summary_exact` | Exact length doesn't add ellipsis |
| `test_contains_math_inline` | `$...$` detected |
| `test_contains_math_block` | `$$...$$` detected |
| `test_contains_math_none` | Plain text returns false |
| `test_extract_wiki_links` | `[[Title]]` extracted from markdown |
| `test_render_code_block` | Code blocks render with `<code>` |
| `test_render_strikethrough` | `~~text~~` renders as `<del>` |

## plugin/host.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `test_empty_plugin_manager` | New manager has no plugins |
| `test_load_nonexistent_directory` | Loading from nonexistent dir returns empty |
| `test_process_all_empty` | Processing with no plugins returns empty |

## watcher.rs (1 test)
| Test | What it verifies |
|------|-----------------|
| `test_watch_directory` | Creating a file triggers a watcher event |

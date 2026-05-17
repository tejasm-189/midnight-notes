# Tests — src/core/

## note.rs (12 tests)
| Test | What it verifies |
|------|-----------------|
| `creating_a_note_with_title_and_content_succeeds` | Create note with title + content |
| `getting_a_note_by_id_returns_correct_note` | Get note by ID |
| `getting_nonexistent_snapshot_returns_none` | Nonexistent ID returns None |
| `updating_note_title_and_content_persists_changes` | Update title and content |
| `trashing_a_note_moves_it_from_active_to_trash` | Trash removes from active, shows in trashed |
| `deleting_a_tag_removes_it_and_note_tags_note` | Permanent delete removes note |
| `toggling_pin_flips_the_pinned_flag` | Toggle pin toggles the pinned flag |
| `pinned_notes_appear_first_in_active_list` | Pinned notes appear first |
| `fts5_search_finds_matching_notes` | FTS5 search finds matching notes |
| `archived_notes_appear_in_archived_list` | Archived notes list |
| `restoring_a_trashed_note_returns_it_to_active` | Trashed → restored → appears in active |
| `deleting_a_tag_removes_it_and_note_tags_permanently` | Permanent delete removes note completely |

## tag.rs (12 tests)
| Test | What it verifies |
|------|-----------------|
| `creating_a_tag_and_getting_by_id_returns_it` | Create tag, get by ID |
| `creating_a_tag_with_parent_creates_hierarchy` | Tag with parent_id creates hierarchy |
| `getting_a_tag_by_name_returns_correct_tag` | Get tag by name |
| `updating_tag_name_and_color_persists_changes` | Update name / color |
| `deleting_a_tag_removes_it_and_note_tags` | Delete removes tag and note_tags |
| `root_tags_have_no_parent` | Root tags (no parent) listed |
| `getting_children_of_a_parent_tag_works` | Children of a parent tag |
| `assigning_tag_to_note_and_listing_tags_works` | Assign tag to note, list tags for note |
| `removing_tag_from_note_clears_it` | Remove tag from note |
| `creating_tag_with_empty_name_is_rejected` | Empty tag name returns error |
| `listing_all_tags_returns_every_tag` | List all tags |

## backlinks.rs (9 tests)
| Test | What it verifies |
|------|-----------------|
| `extracting_wiki_links_from_content_returns_titles` | `[[Title]]` extracted from content |
| `extracting_wiki_links_from_content_returns_titles_no_matches` | No links → empty list |
| `extracting_wiki_links_from_content_returns_titles_empty_content` | Empty string → empty list |
| `extracting_wiki_links_from_content_returns_titles_trims_whitespace` | `[[  Title  ]]` → `Title` |
| `refreshing_backlinks_and_getting_mentions_works` | Refresh backlinks, get linked mentions |
| `getting_outgoing_links_from_a_note_works` | Notes this note links to |
| `refreshing_backlinks_replaces_previous_links` | Refresh replaces old backlinks |
| `resolving_exact_title_match_returns_note_id` | Exact title match resolves to ID |
| `resolving_nonexistent_title_returns_none` | Nonexistent title returns None |

## search.rs (8 tests)
| Test | What it verifies |
|------|-----------------|
| `plain_text_query_has_no_filters` | Plain text query returns no filters |
| `tag_prefix_extracts_tag_filter` | `tag:work` extracts tag filter |
| `has_todo_prefix_extracts_todo_filter` | `has:todo` extracts filter |
| `path_prefix_extracts_path_filter` | `path:docs/` extracts path filter |
| `multiple_filters_are_all_extracted` | Combined filters all extracted |
| `fts5_search_finds_matching_notes_basic` | FTS5 search finds matching notes |
| `saving_listing_and_deleting_smart_views_works` | Save/list/delete smart views |
| `executing_saved_smart_view_returns_results` | Execute saved smart view |

## history.rs (6 tests)
| Test | What it verifies |
|------|-----------------|
| `new_note_has_no_version_history` | New note has no history |
| `updating_a_note_creates_a_snapshot` | Update creates a snapshot |
| `getting_snapshot_by_id_returns_correct_data` | Get snapshot by history ID |
| `restoring_snapshot_reverts_content` | Restore reverts content |
| `diffing_two_snapshots_shows_changes` | Diff between two snapshots |
| `getting_nonexistent_snapshot_returns_none` | Nonexistent snapshot returns None |

## export.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `exporting_then_importing_preserves_content` | Export then import preserves content |
| `wrong_password_fails_import` | Wrong password fails import |
| `exporting_with_no_notes_still_works` | Export with no notes still works |

## markdown.rs (12 tests)
| Test | What it verifies |
|------|-----------------|
| `rendering_bold_markdown_produces_strong_tag` | Bold markdown renders as `<strong>` |
| `rendering_table_produces_th_and_td_tags` | Tables render with `<th>`/`<td>` |
| `rendering_task_list_shows_checked_items` | Task lists render with `checked` |
| `extracting_plain_text_truncates_at_limit` | Text extraction truncates at limit |
| `extracting_plain_text_truncates_at_limit_short` | Short text doesn't truncate |
| `extracting_plain_text_truncates_at_limit_exact` | Exact length doesn't add ellipsis |
| `dollar_signs_detect_inline_math` | `$...$` detected |
| `double_dollar_signs_detect_block_math` | `$$...$$` detected |
| `plain_text_returns_false_for_math_check` | Plain text returns false |
| `extracting_wiki_links_from_markdown_works` | `[[Title]]` extracted from markdown |
| `rendering_code_block_produces_code_tag` | Code blocks render with `<code>` |
| `rendering_strikethrough_produces_del_tag` | `~~text~~` renders as `<del>` strikethrough |

## plugin/host.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `new_plugin_manager_has_no_plugins` | New manager has no plugins |
| `loading_from_nonexistent_directory_returns_empty` | Loading from nonexistent dir returns empty |
| `processing_with_no_plugins_returns_empty` | Processing with no plugins returns empty |

## watcher.rs (1 test)
| Test | What it verifies |
|------|-----------------|
| `creating_a_file_triggers_watcher_event` | Creating a file triggers a watcher event |

# Tests — tests/

## integration.rs (11 tests)
| Test | What it verifies |
|------|-----------------|
| `full_note_lifecycle_create_read_update_pin_archive_trash_delete` | CRUD, pin, archive, trash, delete end-to-end |
| `creating_assigning_and_removing_tags_from_notes` | Tag CRUD, assign to note, list by tag |
| `backlinks_refresh_mentions_outgoing_and_clear` | Backlink refresh, mentions, outgoing, clear |
| `fts5_search_finds_notes_and_smart_views_execute` | FTS5 search + smart view save & execute |
| `version_snapshots_on_update_diff_and_restore` | Snapshots on update, diff, restore |
| `encrypted_zip_export_import_preserves_content` | Export to zip, import back, verify content |
| `markdown_renders_html_and_plain_text_extraction` | HTML rendering, plain text, math check, wiki links |
| `tag_parent_child_hierarchy_update_and_delete` | Parent/child tags, children, roots, update, delete |
| `fts5_search_finds_matching_notes_filters` | Search with tag: filter combined with text |

## Doc-tests (2 tests)
| File | What it verifies |
|------|-----------------|
| `markdown.rs:render_markdown` | Doc example renders heading + bold |
| `keychain.rs:derive_key` | Doc example derives 32-byte key |

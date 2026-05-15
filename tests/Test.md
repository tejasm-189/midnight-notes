# Tests — tests/

## integration.rs (11 tests)
| Test | What it verifies |
|------|-----------------|
| `test_note_lifecycle` | CRUD, pin, archive, trash, delete end-to-end |
| `test_tags_and_notes` | Tag CRUD, assign to note, list by tag |
| `test_backlinks_graph` | Backlink refresh, mentions, outgoing, clear |
| `test_fts5_search` | FTS5 search + smart view save & execute |
| `test_version_history` | Snapshots on update, diff, restore |
| `test_encrypted_export_import` | Export to zip, import back, verify content |
| `test_markdown_rendering` | HTML rendering, plain text, math check, wiki links |
| `test_tag_tree` | Parent/child tags, children, roots, update, delete |
| `test_search_filters` | Search with tag: filter combined with text |

## Doc-tests (2 tests)
| File | What it verifies |
|------|-----------------|
| `markdown.rs:render_markdown` | Doc example renders heading + bold |
| `keychain.rs:derive_key` | Doc example derives 32-byte key |

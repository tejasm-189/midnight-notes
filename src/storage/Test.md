# Tests — src/storage/

## database.rs (4 tests)
| Test | What it verifies |
|------|-----------------|
| `opening_a_new_database_creates_notes_table` | Opening a DB creates the notes table |
| `in_memory_database_works_correctly` | In-memory database works |
| `opening_same_database_file_twice_succeeds` | Opening same file twice succeeds |
| `setting_and_getting_encryption_key_works` | Set/get encryption key, is_encryption_enabled |

## migrations.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `all_migration_tables_are_created` | All tables (notes, tags, FTS5, etc.) created |
| `running_migrations_twice_does_not_fail` | Running migrations twice doesn't fail |
| `schema_version_matches_migration_count` | Schema version matches migration count |

## attachments.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `storing_and_retrieving_attachment_bytes_works` | Store bytes, read them back |
| `deleting_a_tag_removes_it_and_note_tags` | Delete removes the file |
| `getting_nonexistent_attachment_returns_error` | Getting nonexistent file returns error |

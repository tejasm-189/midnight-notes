# Tests — src/storage/

## database.rs (4 tests)
| Test | What it verifies |
|------|-----------------|
| `test_open_and_create` | Opening a DB creates the notes table |
| `test_open_in_memory` | In-memory database works |
| `test_open_twice` | Opening same file twice succeeds |
| `test_encryption_key` | Set/get encryption key, is_encryption_enabled |

## migrations.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `test_migrations_run` | All tables (notes, tags, FTS5, etc.) created |
| `test_migrations_idempotent` | Running migrations twice doesn't fail |
| `test_schema_version` | Schema version matches migration count |

## attachments.rs (3 tests)
| Test | What it verifies |
|------|-----------------|
| `test_store_and_get` | Store bytes, read them back |
| `test_delete` | Delete removes the file |
| `test_not_found` | Getting nonexistent file returns error |

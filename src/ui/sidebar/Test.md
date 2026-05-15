# Tests — src/ui/sidebar/

## Unit Tests
Currently no unit tests for sidebar components (backlinks, tag_tree, smart_views, calendar). SidebarItem component is shared and used by both workspace and settings.

### Components without tests (needs coverage):
- `backlinks.rs` — Linked mentions panel logic
- `tag_tree.rs` — Tag tree rendering and click handling
- `smart_views.rs` — Saved views list
- `calendar.rs` — Date picker month grid, navigation, daily notes

## Running
```bash
# No dedicated tests yet — tested via integration tests
cargo test --test integration
```

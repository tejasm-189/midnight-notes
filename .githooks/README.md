# .githooks/ — Git Hooks

## Files

| File | Status | Description |
|------|--------|-------------|
| `pre-commit` | ✅ done | Checks src/ READMEs exist + updated on .rs changes, runs cargo check + fmt |
| `install.sh` | ✅ done | Symlinks hooks into .git/hooks/ |

## Setup
Run `bash .githooks/install.sh` after cloning to activate hooks.

## Behavior
- Warns if a .rs file is staged but its folder's README.md isn't
- Blocks if cargo check fails
- Blocks if cargo fmt --check fails

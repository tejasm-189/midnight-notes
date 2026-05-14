# Scaffold Template for Any Project

## Root Structure

```
<project-root>/
├── AGENTS.md                  # AI agent rules (REQUIRED)
├── docs/                      # Deep documentation
├── <src-or-similar>/          # Source code
│   ├── README.md              # Folder registry + progress
│   └── <subfolders>/
│       └── README.md          # Subfolder registry + progress
├── .githooks/
│   ├── pre-commit             # Docs/quality checks
│   └── install.sh             # Hook installer
└── <config-files>             # e.g. Cargo.toml, package.json
```

## AGENTS.md Template

```markdown
# AGENTS.md — AI Agent Instructions

## Commands
- Build: `<build-cmd>`
- Test: `<test-cmd>`
- Lint: `<lint-cmd>`
- Typecheck: `<typecheck-cmd>`

## Boundaries
- **Always do**: format, lint, test before finishing; update folder READMEs when files change
- **Ask first**: adding major dependencies, changing architecture, modifying CI
- **Never do**: commit secrets, edit .gitignore without asking, push to main without PR

## Conventions
- Folder-level README.md must list all files and their status (planned/wip/done)
- Document every iteration: after each significant change, update the relevant README
- Keep AGENTS.md under 80 lines — use docs/ for depth

## Architecture
See `docs/architecture.md` for the full picture.
```

## Folder-Level README.md Template

```markdown
# <Folder Name>

## Purpose
<What this folder contains and why>

## Files

| File | Status | Description |
|------|--------|-------------|
| `main.rs` | done | Entry point |
| `config.rs` | wip | Configuration parsing |
| ... | planned | ... |

## Progress
- [x] Entry point
- [ ] Config module
- [ ] Database layer
- [ ] Tests

## Notes
<Ad-hoc decisions, gotchas, links>
```

## Pre-commit Hook (`.githooks/pre-commit`)

```bash
#!/bin/sh
set -e

echo "=== Pre-commit Hook ==="

# Check folder READMEs exist for all src directories
# (customize the path pattern to your project)
find <src-dir> -type d ! -path '*/node_modules/*' ! -path '*/target/*' | while read dir; do
    if [ ! -f "$dir/README.md" ]; then
        # Only warn, don't block — create it automatically
        if ! ls "$dir"/*.rs "$dir"/*.ts "$dir"/*.js "$dir"/*.py 2>/dev/null >/dev/null; then
            continue  # skip dirs without source files
        fi
        echo "WARNING: $dir/README.md missing" >&2
    fi
done

# Run quality checks (customize)
# <build-cmd>
# <lint-cmd>

echo "=== Pre-commit Passed ==="
```

## Hook Installer (`scripts/install-hooks.sh`)

```bash
#!/bin/sh
set -e

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)/.githooks"

for hook in "$SCRIPT_DIR"/*; do
    hook_name="$(basename "$hook")"
    ln -sf "$hook" "$HOOK_DIR/$hook_name"
    echo "Installed hook: $hook_name"
done
```

## Iteration Workflow

1. **Start iteration** → create/update folder README with planned files
2. **Write code** → update README status (planned → wip → done)
3. **Verify** → build, lint, test, typecheck
4. **Commit** → pre-commit hook enforces README accuracy
5. **Log** → add key decisions to the Notes section of the relevant README

## Progress Tracking

Use this in each folder README:

| File | Status | Description |
|------|--------|-------------|
| `path/to/file.ext` | ✅ done / 🔄 wip / 📋 planned | What it does |

Update statuses after every code change — this is your "source of truth" for what's built.
```


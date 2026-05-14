#!/bin/sh
set -e

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

for hook in "$SCRIPT_DIR"/*; do
    hook_name="$(basename "$hook")"
    [ "$hook_name" = "install.sh" ] && continue
    ln -sf "$hook" "$HOOK_DIR/$hook_name"
    echo "Installed hook: $hook_name"
done

echo "Git hooks installed successfully."

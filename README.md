# Notes

A cross-platform notes application built with Rust — zero-knowledge encrypted, local-first, feature-rich. Inspired by Standard Notes, built for developers.

## Core Philosophy

**Privacy by default.** Your notes are encrypted at rest and the app never phones home. All data lives on your machine unless you explicitly export it.

## Features

### Security & Privacy
- **XChaCha20-Poly1305 encryption** — entire SQLite database encrypted via SQLCipher, keys derived from master password using **Argon2id**
- **Zero-knowledge architecture** — no accounts, no cloud, no telemetry
- **Encrypted exports** — password-protected `.zip` or custom encrypted container for portable backups
- **Biometric lock** — native OS biometric prompts (Windows Hello / macOS Touch ID / Linux PAM) to unlock the app database via Tauri system APIs

### Advanced Organization
- **Bidirectional linking** — `[[Note Name]]` syntax creates backlinks; the sidebar shows "Linked Mentions" from any note referencing the current one
- **Nested tags & virtual folders** — `work/project-a` style tags render as a tree in the Dioxus sidebar; virtual tags are live-saved searches (e.g. a "Todo" folder showing all notes containing `- [ ]`)
- **Smart views** — saved FTS5 queries as persistent folder-like filters

### High-Feature Editor
- **Multi-mode editing:**
  - *Prose mode* — clean, distraction-free Markdown writing with live preview
  - *Code mode* — integrated tree-sitter syntax highlighting for code-heavy notes
  - *Vim mode* — modal editing with partial `.vimrc` support via `kanban`-style bindings
- **LaTeX typesetting** — inline and block math rendered via KaTeX
- **Slash commands** — type `/` to trigger a command palette for inserting tables, code blocks, images, and more
- **Markdown rendering** — `pulldown-cmark` (fastest Rust parser) turns raw text into Dioxus components in real-time

### Portability & Extensions
- **Version history** — SQLite stores snapshots or diffs on every save; full revision tree with diff view
- **Plugin system** — write plugins in WASM that process note text (currency conversion, inline math evaluation, custom formatters)
- **Attachment handling** — images/PDFs stored as blobs in a hidden `.attachments/` directory, linked via SQLite — keeps `.md` files clean and portable
- **File watching** — `notify` crate watches your notes directory; changes from external editors (VS Code, etc.) sync back into the app instantly

### Quality of Life
- **Note pinning** — pin important notes to the top of the list
- **Archiving** — hide notes without deleting them (with "Show Archived" toggle)
- **Read time & word count** — subtle footer metadata on every note
- **Theming** — Dark, Light, and Midnight (OLED) modes via Tailwind CSS

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop shell | Tauri (native APIs, biometrics, file system) |
| Frontend | Dioxus (renders inside Tauri webview) |
| Backend logic | Rust core crate (MVC pattern) |
| Storage | SQLCipher (encrypted SQLite) |
| Full-text search | SQLite FTS5 extension |
| Markdown parsing | `pulldown-cmark` |
| Syntax highlighting | `tree-sitter` (code mode) |
| File watching | `notify` crate |
| Math rendering | KaTeX (LaTeX support) |
| Key derivation | `Argon2id` via `argon2` crate |
| WASM runtime | `wasmtime` (plugin system) |
| CSS | Tailwind CSS |
| Cross-platform | Linux / macOS / Windows (desktop) |

## Architecture

```
┌─────────────────────────────────────┐
│           Tauri Desktop Shell       │
│  (biometrics, file system, menus)   │
│  ┌───────────────────────────────┐  │
│  │        Dioxus Frontend        │  │
│  │  Editor / Viewer / Sidebar    │  │
│  └──────────┬────────────────────┘  │
└─────────────┼───────────────────────┘
              │ IPC (Tauri commands)
┌─────────────▼───────────────────────┐
│          Core Crate (MVC)           │
│  ┌─────────┐ ┌────────┐ ┌───────┐  │
│  │ Note    │ │ Tag    │ │Search │  │
│  │ Service │ │ Service│ │Engine │  │
│  └────┬────┘ └───┬────┘ └───┬───┘  │
│       │          │          │      │
│  ┌────▼──────────▼──────────▼───┐  │
│  │      SQLCipher / SQLite      │  │
│  │  notes.db (encrypted)        │  │
│  │  + FTS5 virtual tables       │  │
│  └──────────────────────────────┘  │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  .attachments/ (blobs)      │  │
│  │  .snapshots/ (history)      │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

## Project Structure

```
notes/
├── AGENTS.md                  # AI agent instructions
├── README.md                  # This file
├── docs/                      # Deep documentation
│   ├── architecture.md        # Full architecture deep-dive
│   ├── security.md            # Encryption model details
│   ├── plugin-dev.md          # WASM plugin API docs
│   └── theming.md             # Theme customization guide
├── src/
│   ├── README.md              # Folder registry + progress
│   │
│   ├── main.rs                # Tauri entry point
│   ├── lib.rs                 # Shared library root
│   │
│   ├── ui/                    # Dioxus components (frontend)
│   │   ├── README.md
│   │   ├── mod.rs
│   │   ├── app.rs             # Root component + routing
│   │   ├── editor/            # Multi-mode editor
│   │   │   ├── mod.rs
│   │   │   ├── prose.rs       # Prose mode (markdown)
│   │   │   ├── code.rs        # Code mode (tree-sitter)
│   │   │   ├── vim.rs         # Vim mode bindings
│   │   │   └── slash.rs       # Slash command palette
│   │   ├── viewer.rs          # Note viewer/reader
│   │   ├── sidebar/           # Navigation sidebar
│   │   │   ├── mod.rs
│   │   │   ├── tag_tree.rs    # Nested tag tree
│   │   │   ├── smart_views.rs # Saved search views
│   │   │   └── backlinks.rs   # Linked mentions
│   │   ├── search.rs          # Search bar + results
│   │   ├── settings.rs        # Settings panel
│   │   └── theme.rs           # Theme switcher
│   │
│   ├── core/                  # Business logic (MVC Model)
│   │   ├── README.md
│   │   ├── mod.rs
│   │   ├── note.rs            # Note model + CRUD service
│   │   ├── tag.rs             # Tag model + tree operations
│   │   ├── search.rs          # FTS5 search engine
│   │   ├── backlinks.rs       # Bidirectional link resolver
│   │   ├── history.rs         # Version snapshots + diff
│   │   ├── export.rs          # Encrypted export logic
│   │   └── plugin/            # WASM plugin host
│   │       ├── mod.rs
│   │       ├── host.rs        # WASM runtime (wasmtime)
│   │       └── api.rs         # Plugin API surface
│   │
│   ├── storage/               # Data persistence
│   │   ├── README.md
│   │   ├── mod.rs
│   │   ├── database.rs        # SQLCipher connection + key derivation
│   │   ├── migrations.rs      # Schema migrations
│   │   ├── models.rs          # Row ↔ struct mappings
│   │   └── attachments.rs     # Blob file management
│   │
│   └── crypto/                # Encryption utilities
│       ├── README.md
│       ├── mod.rs
│       ├── keychain.rs        # Argon2id key derivation
│       └── cipher.rs          # XChaCha20-Poly1305 helpers
│
├── plugins/                   # User-installed WASM plugins
│   └── example-plugin/        # Example plugin project
│
├── .githooks/
│   ├── pre-commit
│   └── install.sh
├── styles/
│   ├── app.css                # Base styles
│   ├── light.css              # Light theme vars
│   ├── dark.css               # Dark theme vars
│   └── midnight.css           # Midnight (OLED) theme vars
├── Tauri.toml                 # Tauri configuration
└── Cargo.toml
```

## Testing

```bash
# Run all tests (unit + integration)
cargo test

# Run integration tests only
cargo test --test integration

# Run specific test
cargo test test_note_lifecycle

# Run tests with output
cargo test -- --nocapture

# Code coverage (install tarpaulin first)
# cargo install cargo-tarpaulin
# cargo tarpaulin --ignore-tests
```

## Getting Started

```bash
# Prerequisites
# - Rust 1.70+
# - SQLCipher dev libraries (libsqlcipher-dev on Debian, sqlcipher on macOS)
# - Tauri system dependencies (webkit2gtk on Linux)

# Clone & run
git clone <repo>
cd notes
cargo tauri dev

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt -- --check

# Build for production
cargo tauri build
```

## Development Phases

| Phase | Scope | Status |
|-------|-------|--------|
| 1 | Project scaffold, SQLite schema, note CRUD, encryption utilities | ✅ Done |
| 2 | Dioxus UI shell (vault lock + workspace), screen routing | ✅ Done |
| 3 | Tags (nested), backlinks (`[[...]]`), smart views, FTS5 search | ✅ Done |
| 4 | Multi-mode editor (prose/code/vim), slash commands, KaTeX math | ✅ Done |
| 5 | Version history + diff, encrypted exports, attachment blobs | ✅ Done |
| 6 | WASM plugin system, file watching (notify), biometric lock | ✅ Done |
| 7 | Theming (light/dark/midnight), settings UI, polish | ✅ Done |
| 8 | CI pipeline (GitHub Actions), release profile, cross-compile config | ✅ Done |

## Security Model

- **Database**: SQLCipher encrypts the entire SQLite file with XChaCha20-Poly1305
- **Key derivation**: Master password → Argon2id (memory-hard, tunable params) → 256-bit encryption key
- **Key storage**: Derived key held only in memory; never written to disk
- **Exports**: Password-protected containers with separate Argon2id-derived key
- **No telemetry**: Zero network requests unless user explicitly exports

## License

MIT

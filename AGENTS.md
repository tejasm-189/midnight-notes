# AGENTS.md — Midnight Notes

## Commands
- Build desktop: `cargo run`
- Test all: `cargo test`
- Test specific: `cargo test test_name` or `cargo test --test integration`
- Test with output: `cargo test -- --nocapture`
- Test single module: `cargo test core::note::tests`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt -- --check`
- Build release: `cargo tauri build`

## Boundaries
- **Always do**: format, lint, test; update folder READMEs + Test.md when files change; run `cargo check` before committing
- **Ask first**: adding major dependencies, changing DB schema, modifying encryption model, adding network requests
- **Never do**: commit secrets, hardcode test credentials, disable encryption, push to main without PR

## Architecture
See `docs/architecture.md` for full details. See `docs/Design/` for UI mockups. See `DESIGN.md` for design tokens.

## Conventions
- Folder-level `README.md` must list all files and their status (planned/wip/done)
- Folder-level `Test.md` must list all unit and integration tests, organized by file, with descriptions
- **Every time you add/modify/remove a test**, update the relevant `Test.md`
- **Every time you modify a .rs file**, ensure the folder's `README.md` and `Test.md` are also staged
- **When running tests, always reference the Test.md of the folder you're working in** — it lists every test with descriptions
- For example: modifying `src/core/note.rs` → check `src/core/Test.md` → run `cargo test creating_a_note_with_title_and_content_succeeds updating_note_title_and_content_persists_changes`
- **Test names are descriptive sentences** — use them directly as test selectors:
  ```bash
  cargo test wrong_password_fails_import
  cargo test trashing_a_note_moves_it_from_active_to_trash
  cargo test pressing_escape_switches_insert_to_normal_mode
  ```
- Document every iteration: after each significant change, update the relevant README/Test.md
- All markdown rendering uses `pulldown-cmark`
- All encryption uses XChaCha20-Poly1305 + Argon2id key derivation
- Dioxus components live in `src/ui/` organized by feature
- Core business logic in `src/core/` — no Dioxus imports allowed
- Storage layer in `src/storage/` — no business logic
- Crypto utilities in `src/crypto/` — pure functions, no I/O

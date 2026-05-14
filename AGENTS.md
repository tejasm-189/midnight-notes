# AGENTS.md — Midnight Notes

## Commands
- Build desktop: `cargo tauri dev`
- Build web: `cargo build --target wasm32-unknown-unknown`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt -- --check`
- Build release: `cargo tauri build`

## Boundaries
- **Always do**: format, lint, test; update folder READMEs when files change; run `cargo check` before committing
- **Ask first**: adding major dependencies, changing DB schema, modifying encryption model, adding network requests
- **Never do**: commit secrets, hardcode test credentials, disable SQLCipher encryption, push to main without PR

## Architecture
See `docs/architecture.md` for full details. See `docs/Design/` for UI mockups. See `DESIGN.md` for design tokens.

## Conventions
- Folder-level `README.md` must list all files and their status (planned/wip/done)
- Document every iteration: after each significant change, update the relevant README
- All markdown rendering uses `pulldown-cmark`
- All encryption uses XChaCha20-Poly1305 via SQLCipher + Argon2id key derivation
- Dioxus components live in `src/ui/` organized by feature
- Core business logic in `src/core/` — no Dioxus imports allowed
- Storage layer in `src/storage/` — SQLCipher only, no business logic
- Crypto utilities in `src/crypto/` — pure functions, no I/O

# src/crypto/ — Encryption Utilities

## Purpose
Pure encryption/decryption functions. No I/O, no database access. Provides key derivation (Argon2id), symmetric encryption (XChaCha20-Poly1305), and helpers for encrypted exports.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | 📋 planned | Module root, re-exports |
| `keychain.rs` | ✅ done | Argon2id key derivation |
| `cipher.rs` | ✅ done | XChaCha20-Poly1305 + AES-256-GCM encrypt/decrypt |

## Progress
- [x] Argon2id key derivation
- [x] XChaCha20-Poly1305 encryption
- [x] AES-256-GCM encryption
- [x] Unit tests (7 tests)

## Notes
- Must be pure functions — no I/O, no side effects
- All functions return `Result<T, CryptoError>`
- Key material never logged or printed

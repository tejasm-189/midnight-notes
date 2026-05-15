# src/crypto/ — Encryption Utilities

## Purpose
Pure encryption/decryption functions. No I/O, no database access. Provides key derivation (Argon2id), symmetric encryption (XChaCha20-Poly1305, AES-256-GCM), and helpers for encrypted exports.

## Files

| File | Status | Description |
|------|--------|-------------|
| `mod.rs` | ✅ done | Module root |
| `keychain.rs` | ✅ done | Argon2id key derivation (m=65536, t=3, p=4), salt generation, password validation |
| `cipher.rs` | ✅ done | XChaCha20-Poly1305 + AES-256-GCM encrypt/decrypt |

## Progress
- [x] Argon2id key derivation
- [x] XChaCha20-Poly1305 encrypt/decrypt
- [x] AES-256-GCM encrypt/decrypt
- [x] Unit tests (7 tests)

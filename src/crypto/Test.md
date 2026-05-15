# Tests — src/crypto/

## keychain.rs (7 tests)
| Test | What it verifies |
|------|-----------------|
| `test_derive_key_deterministic` | Same password + salt → same key |
| `test_derive_key_different_salt` | Different salt → different key |
| `test_derive_key_length` | Output is 32 bytes |
| `test_generate_salt_length` | Salt is 16 bytes |
| `test_generate_salt_unique` | Consecutive salts differ |
| `test_validate_password_strength_valid` | Password ≥ 8 chars accepted |
| `test_validate_password_strength_short` | Password < 8 chars rejected |

## cipher.rs (5 tests)
| Test | What it verifies |
|------|-----------------|
| `test_xchacha20_roundtrip` | Encrypt then decrypt returns original |
| `test_xchacha20_wrong_key` | Wrong key fails decryption |
| `test_xchacha20_unique_nonce` | Same plaintext produces different ciphertext |
| `test_aes256gcm_roundtrip` | AES-256-GCM encrypt/decrypt roundtrip |
| `test_encrypt_empty` | Empty plaintext roundtrips successfully |

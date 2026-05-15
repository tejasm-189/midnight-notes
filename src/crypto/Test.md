# Tests — src/crypto/

## keychain.rs (7 tests)
| Test | What it verifies |
|------|-----------------|
| `same_password_and_salt_produce_same_key` | Same password + salt → same key |
| `different_salts_produce_different_keys` | Different salt → different key |
| `derived_key_is_32_bytes` | Output is 32 bytes |
| `generated_salt_is_16_bytes` | Salt is 16 bytes |
| `consecutive_salts_are_unique` | Consecutive salts differ |
| `password_with_8_or_more_chars_is_accepted` | Password ≥ 8 chars accepted |
| `password_with_less_than_8_chars_is_rejected` | Password < 8 chars rejected |

## cipher.rs (5 tests)
| Test | What it verifies |
|------|-----------------|
| `xchacha20_encrypt_then_decrypt_returns_original` | Encrypt then decrypt returns original |
| `xchacha20_with_wrong_key_fails_decryption` | Wrong key fails decryption |
| `xchacha20_produces_different_ciphertext_for_same_plaintext` | Same plaintext produces different ciphertext |
| `aes256gcm_encrypt_then_decrypt_returns_original` | AES-256-GCM encrypt/decrypt roundtrip |
| `encrypting_and_decrypting_empty_data_succeeds` | Empty plaintext roundtrips successfully |

# About OpenTitan `cryptolib`

This directory contains the interface and data type definitions for the OpenTitan cryptographic library. The library provides a wide range of cryptographic primitives, including symmetric encryption, message authentication, asymmetric cryptography, hashing, key derivation, and random number generation.

## Capabilities Summary

### **1. Symmetric Encryption (AES)**
The library supports various AES modes and padding schemes, providing both one-shot and streaming interfaces.

*   **Algorithms/Modes**: ECB, CBC, CFB, OFB, CTR, GCM, KWP (Key Wrap with Padding).
*   **Padding Schemes**: PKCS7, ISO9797M2, Null.
*   **Functions**:
    *   **General AES**: `aes`, `aes_padded_plaintext_length`.
    *   **AES-GCM (One-shot)**: `aes_gcm_encrypt`, `aes_gcm_decrypt`.
    *   **AES-GCM (Streaming)**: `aes_gcm_encrypt_init`, `aes_gcm_decrypt_init`, `aes_gcm_update_aad`, `aes_gcm_update_encrypted_data`, `aes_gcm_encrypt_final`, `aes_gcm_decrypt_final`.

### **2. Message Authentication Codes (HMAC & KMAC)**
*   **HMAC Modes**: SHA2-256, SHA2-384, SHA2-512.
*   **KMAC Modes**: KMAC-128, KMAC-256.
*   **Functions**:
    *   **HMAC (One-shot)**: `hmac`.
    *   **HMAC (Streaming)**: `hmac_init`, `hmac_update`, `hmac_final`.
    *   **KMAC**: `kmac`.

### **3. Asymmetric Cryptography (ECC & RSA)**
The library provides comprehensive support for Elliptic Curve and RSA algorithms, including specialized DICE-backed key support.

*   **Curves**: P-256, P-384, Ed25519, X25519.
*   **RSA Sizes**: 2048, 3072, 4096 bits.
*   **RSA Modes**: Sign PKCS#1 v1.5, Sign PSS, Encrypt OAEP.
*   **Functions**:
    *   **ECDSA (P-256/P-384)**: `keygen`, `sign`, `verify`, `sign_verify` (with fault injection mitigation).
    *   **ECDH (P-256/P-384)**: `keygen`, `ecdh_p256`/`ecdh_p384` (shared secret generation).
    *   **Ed25519/X25519**: `keygen`, `sign`, `verify`, `x25519` (shared secret).
    *   **DICE (P-256)**: `dice_p256_keygen`, `dice_p256_sign`, `dice_p256_verify`.
    *   **RSA**: `rsa_keygen`, `rsa_public_key_construct`, `rsa_private_key_from_exponents`, `rsa_keypair_from_cofactor`, `rsa_sign`, `rsa_verify`, `rsa_encrypt`, `rsa_decrypt`.

### **4. Hashing (SHA-2 & SHA-3)**
*   **SHA-2 Variants**: 256, 384, 512.
*   **SHA-3 Variants**: 224, 256, 384, 512.
*   **XOFs**: SHAKE128, SHAKE256, cSHAKE128, cSHAKE256.
*   **Functions**:
    *   **SHA-2 (One-shot)**: `sha2_256`, `sha2_384`, `sha2_512`.
    *   **SHA-2 (Streaming)**: `sha2_init`, `sha2_update`, `sha2_final`.
    *   **SHA-3/XOF (One-shot)**: `sha3_224`, `sha3_256`, `sha3_384`, `sha3_512`, `shake128`, `shake256`, `cshake128`, `cshake256`.

### **5. Key Derivation Functions (KDF)**
*   **Algorithms**: HKDF (RFC 5869), KDF-CTR (NIST SP 800-108r1), KMAC-KDF.
*   **Functions**:
    *   **HKDF**: `hkdf`, `hkdf_extract`, `hkdf_expand`.
    *   **KDF-CTR**: `kdf_ctr_hmac`.
    *   **KMAC-KDF**: `kmac_kdf`.

### **6. Random Number Generation (DRBG)**
*   **Functions**: `drbg_instantiate`, `drbg_reseed`, `drbg_manual_instantiate`, `drbg_manual_reseed`, `drbg_generate`, `drbg_manual_generate`, `drbg_uninstantiate`.

### **7. Key Management**
*   **Functions**:
    *   **Generation**: `symmetric_keygen`, `hw_backed_key` (creates handles for hardware-protected keys).
    *   **Wrapping**: `key_wrap`, `key_unwrap`, `wrapped_key_len` (using AES-KWP).
    *   **Import/Export**: `import_blinded_key`, `export_blinded_key` (handling keys in shares).

### **8. Miscellaneous**
*   **Functions**: `build_info` (retrieves version and build hash information).

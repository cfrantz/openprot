
use crate::datatypes::*;

#[allow(unused_variables)]
pub trait CryptoInterface {
    fn aes_padded_plaintext_length(
        plaintext_len: usize,
        aes_padding: AesPadding,
        padded_len: &mut usize,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the AES operation.
    ///
    /// The input data in the `cipher_input` is first padded using the
    /// `aes_padding` scheme and the output is copied to `cipher_output`.
    ///
    /// The caller should allocate space for the `cipher_output` buffer, which is
    /// given in bytes by `otcrypto_aes_padded_plaintext_length`, and set the number
    /// of bytes allocated in the `len` field of the output.  If the user-set length
    /// and the expected length do not match, an error message will be returned.
    ///
    /// Note that, during decryption, the padding mode is ignored. This function
    /// will NOT check the padding or return an error if the padding is invalid,
    /// since doing so could expose a padding oracle (especially in CBC mode).
    ///
    /// @param key Pointer to the blinded key struct with key shares.
    /// @param iv Initialization vector, used for CBC, CFB, OFB, CTR modes. May be
    ///           NULL if mode is ECB.
    /// @param aes_mode Required AES mode of operation.
    /// @param aes_operation Required AES operation (encrypt or decrypt).
    /// @param cipher_input Input data to be ciphered.
    /// @param aes_padding Padding scheme to be used for the data.
    /// @param[out] cipher_output Output data after cipher operation.
    /// @return The result of the cipher operation.
    fn aes(
        key: &mut BlindedKey,
        iv: &mut [u8],
        aes_mode: AesMode,
        aes_operation: AesOperation,
        cipher_input: &[u8],
        aes_padding: AesPadding,
        cipher_output: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the AES-GCM authenticated encryption operation.
    ///
    /// This function encrypts the input `plaintext` to produce an
    /// output `ciphertext`. Together it generates an authentication
    /// tag `auth_tag` on the ciphered data and any non-confidential
    /// additional authenticated data `aad`.
    ///
    /// The caller should allocate space for the `ciphertext` buffer,
    /// (same length as input), `auth_tag` buffer (same as tag_len), and
    /// set the length of expected outputs in the `len` field of
    /// `ciphertext` and `auth_tag`. If the user-set length and the output
    /// length does not match, an error message will be returned.
    ///
    /// @param key Pointer to the blinded gcm-key struct.
    /// @param plaintext Input data to be encrypted and authenticated.
    /// @param iv Initialization vector for the encryption function.
    /// @param aad Additional authenticated data.
    /// @param tag_len Length of authentication tag to be generated.
    /// @param[out] ciphertext Encrypted output data, same length as input data.
    /// @param[out] auth_tag Generated authentication tag.
    /// @return Result of the authenticated encryption.
    /// operation
    fn aes_gcm_encrypt(
        key: &mut BlindedKey,
        plaintext: &[u8],
        iv: &[u8],
        aad: &[u8],
        tag_len: AesGcmTagLen,
        ciphertext: &mut [u8],
        auth_tag: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the AES-GCM authenticated decryption operation.
    ///
    /// This function first verifies if the authentication tag `auth_tag`
    /// matches the internally generated tag. Upon verification, the
    /// function decrypts the input `ciphertext` to get a `plaintext data.
    ///
    /// The caller should allocate space for the `plaintext` buffer,
    /// (same length as ciphertext), and set the length of expected output
    /// in the `len` field of `plaintext`. If the user-set length and the
    /// output length does not match, an error message will be returned.
    ///
    /// The caller must check the `success` argument before operating on
    /// `plaintext`. If the authentication check fails, then `plaintext` should not
    /// be used and there are no guarantees about its contents.
    ///
    /// @param key Pointer to the blinded gcm-key struct.
    /// @param ciphertext Input data to be decrypted.
    /// @param iv Initialization vector for the decryption function.
    /// @param aad Additional authenticated data.
    /// @param tag_len Length of authentication tag to be generated.
    /// @param auth_tag Authentication tag to be verified.
    /// @param[out] plaintext Decrypted plaintext data, same len as input data.
    /// @param[out] success True if the authentication check passed, otherwise false.
    /// @return Result of the authenticated decryption.
    /// operation
    fn aes_gcm_decrypt(
        key: &mut BlindedKey,
        ciphertext: &[u8],
        iv: &[u8],
        aad: &[u8],
        tag_len: AesGcmTagLen,
        auth_tag: &[u8],
        plaintext: &mut [u8],
        success: &mut HardenedBool,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Initializes the AES-GCM authenticated encryption operation.
    ///
    /// The order of operations for encryption is:
    ///   - `otcrypto_aes_gcm_encrypt_init()` called once
    ///   - `otcrypto_aes_gcm_update_aad()` called zero or more times
    ///   - `otcrypto_aes_gcm_update_encrypted_data()` called zero or more times
    ///   - `otcrypto_aes_gcm_encrypt_final()` called once
    ///
    /// Associated data must be added first, before encrypted data; the caller may
    /// not call `otcrypto_aes_gcm_udpate_aad()` after the first call to
    /// `otcrypto_aes_gcm_update_encrypted_data()`.
    ///
    /// The resulting AES-GCM context will include pointers into the keyblob of the
    /// blinded key. It is important that the blinded key (or at least the keyblob)
    /// remains live as long as `ctx` is. The IV is safe to free.
    ///
    /// @param key Pointer to the blinded key struct.
    /// @param iv Initialization vector for the encryption function.
    /// @param[out] ctx Context object for the operation.
    /// @return Result of the initialization operation.
    fn aes_gcm_encrypt_init(
        key: &mut BlindedKey,
        iv: &[u8],
        ctx: &mut AesGcmContext,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Initializes the AES-GCM authenticated decryption operation.
    ///
    /// The order of operations for decryption is:
    ///   - `otcrypto_aes_gcm_decrypt_init()` called once
    ///   - `otcrypto_aes_gcm_update_aad()` called zero or more times
    ///   - `otcrypto_aes_gcm_update_encrypted_data()` called zero or more times
    ///   - `otcrypto_aes_gcm_decrypt_final()` called once
    ///
    /// Associated data must be added first, before encrypted data; the caller may
    /// not call `otcrypto_aes_gcm_udpate_aad()` after the first call to
    /// `otcrypto_aes_gcm_update_encrypted_data()`.
    ///
    /// The resulting AES-GCM context will include pointers into the keyblob of the
    /// blinded key. It is important that the blinded key (or at least the keyblob)
    /// remains live as long as `ctx` is. The IV is safe to free.
    ///
    /// IMPORTANT: Although this routine produces decrypted data incrementally, it
    /// is the caller's responsibility to ensure that they do not trust the
    /// decrypted data until the tag check passes.
    ///
    /// @param key Pointer to the blinded key struct.
    /// @param iv Initialization vector for the decryption function.
    /// @param[out] ctx Context object for the operation.
    /// @return Result of the initialization operation.
    fn aes_gcm_decrypt_init(
        key: &mut BlindedKey,
        iv: &[u8],
        ctx: &mut AesGcmContext,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Updates additional authenticated data for an AES-GCM operation.
    ///
    /// May be used for either encryption or decryption. Call
    /// `otcrypto_aes_gcm_encrypt_init` or `otcrypto_aes_gcm_decrypt_init` first.
    ///
    /// @param ctx Context object for the operation, updated in place.
    /// @param aad Additional authenticated data.
    /// @return Result of the update operation.
    fn aes_gcm_update_aad(ctx: &mut AesGcmContext, aad: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Updates authenticated-and-encrypted data for an AES-GCM operation.
    ///
    /// May be used for either encryption or decryption. Call
    /// `otcrypto_aes_gcm_encrypt_init` or `otcrypto_aes_gcm_decrypt_init` first.
    ///
    /// The caller should allocate space for the output and set the `len` field
    /// accordingly.
    ///
    /// For encryption, `input` is the plaintext and `output` is the ciphertext; for
    /// decryption, they are reversed. The output must always be long enough to
    /// store all full 128-bit blocks of encrypted data received so far minus all
    /// output produced so far; rounding the input length to the next 128-bit
    /// boundary is always enough, but if the caller knows the exact byte-length of
    /// input so far they can calculate it exactly. Returns an error if `output` is
    /// not long enough; if `output` is overly long, only the first
    /// `output_bytes_written` bytes will be used.
    ///
    /// @param ctx Context object for the operation, updated in place.
    /// @param input Plaintext for encryption, ciphertext for decryption.
    /// @param[out] output Ciphertext for encryption, plaintext for decryption.
    /// @param[out] output_bytes_written Number of bytes written to `output`.
    /// @return Result of the update operation.
    fn aes_gcm_update_encrypted_data(
        ctx: &mut AesGcmContext,
        input: &[u8],
        output: &mut [u8],
        output_bytes_written: &mut usize,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Finishes the AES-GCM authenticated encryption operation.
    ///
    /// Processes any remaining plaintext from the context and computes the
    /// authentication tag and up to 1 block of ciphertext.
    ///
    /// The caller should allocate space for the ciphertext and tag buffers and set
    /// the `len` fields accordingly. This function returns the
    /// `ciphertext_bytes_written` parameter with the number of bytes written to
    /// `ciphertext`, which is always either 16 or 0. This function returns an error
    /// if the ciphertext or tag buffer is not long enough.
    ///
    /// @param ctx Context object for the operation.
    /// @param tag_len Length of authentication tag to be generated.
    /// @param[out] ciphertext Encrypted output data.
    /// @param[out] ciphertext_bytes_written Number of bytes written to `ciphertext`.
    /// @param[out] auth_tag Generated authentication tag.
    /// @return Result of the final operation.
    fn aes_gcm_encrypt_final(
        ctx: &mut AesGcmContext,
        tag_len: AesGcmTagLen,
        ciphertext: &mut [u8],
        ciphertext_bytes_written: &mut usize,
        auth_tag: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Finishes the AES-GCM authenticated decryption operation.
    ///
    /// Processes any remaining ciphertext from the context and computes the
    /// authentication tag and up to 1 block of plaintext.
    ///
    /// The caller should allocate space for the plaintext buffer and set the `len`
    /// field accordingly. This function returns the `ciphertext_bytes_written`
    /// parameter with the number of bytes written to `ciphertext`. This function
    /// returns an error if the plaintext buffer is not long enough.
    ///
    /// IMPORTANT: the caller must check both the returned status and the `success`
    /// parameter to know if the tag is valid. The returned status may be OK even if
    /// the tag check did not succeed, if there were no errors during processing.
    ///
    /// @param ctx Context object for the operation.
    /// @param auth_tag Authentication tag to check.
    /// @param tag_len Length of authentication tag.
    /// @param[out] plaintext Decrypted output data.
    /// @param[out] plaintext_bytes_written Number of bytes written to `plaintext`.
    /// @param[out] success Whether the tag passed verification.
    /// @return Result of the final operation.
    fn aes_gcm_decrypt_final(
        ctx: &mut AesGcmContext,
        auth_tag: &[u8],
        tag_len: AesGcmTagLen,
        plaintext: &mut [u8],
        plaintext_bytes_written: &mut usize,
        success: &mut HardenedBool,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Read the cryptolib build information.
    ///
    /// Returns the current version of the cryptolib as well as the
    /// latest git commit hash of the `sw/device/lib/crypto` directory.
    ///
    /// @param ctx Pointer to the generic HMAC context struct.
    /// @param[out] version The current version of the cryptolib.
    /// @param[out] build_hash_low The low portion of the git commit hash of
    /// `sw/device/lib/crypto`.
    /// @param[out] build_hash_high The high portion of the git commit hash of
    /// `sw/device/lib/crypto`.
    /// @return Result of the HMAC final operation.
    fn build_info(
        version: &mut u32,
        released: &mut u8,
        build_hash_low: &mut u32,
        build_hash_high: &mut u32,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generate an ECDSA P256 key from the DICE attestation keymgr.
    ///
    /// @param private_key A blinded key with a keyblob of `dice_diversifier_t`.
    /// @param public_key[out] An unblinded key with a `key` pointer to a 64-byte
    ///        buffer to receive the P256 x/y coordinates.
    /// @return OTCRYPTO_OK.
    fn dice_p256_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Sign a message with an ECDSA P256 key from the DICE attestation keymgr.
    ///
    /// @param private_key A blinded key with a keyblob of `dice_diversifier_t`.
    /// @param message_digest A SHA256 hash of the message to sign.
    /// @param signature[out] The resulting signature.
    /// @return OTCRYPTO_OK.
    fn dice_p256_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Verify a message with an ECDSA P256 key from the DICE attestation keymgr.
    ///
    /// Note: this is here as a debugging aide.  You should really use
    /// `otcrypto_p256_verify` to verify signatures.  If you use this function,
    /// you must check recovered_r to know if the signature was valid.
    ///
    /// @param private_key A blinded key with a keyblob of `dice_diversifier_t`.
    /// @param message_digest The SHA256 hash of the message.
    /// @param signature The signature to verify.
    /// @param recovered_r The recovered R portion of the signature.
    /// @return OTCRYPTO_OK.
    fn dice_p256_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
        recovered_r: &mut u32,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Instantiates the DRBG system.
    ///
    /// Initializes the DRBG and the context for DRBG. Gets the required entropy
    /// input automatically from the entropy source.
    ///
    /// The personalization string may empty, and may be up to 48 bytes long; any
    /// longer will result in an error. If the string is word aligned and the size
    /// is a multiple of the word length (32-bit), it is handled using SCA hardened
    /// memory operations. If not, a non SCA hardened fallback is used.
    ///
    /// @param perso_string Pointer to personalization bitstring.
    /// @return Result of the DRBG instantiate operation.
    fn drbg_instantiate(perso_string: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Reseeds the DRBG with fresh entropy.
    ///
    /// Reseeds the DRBG with fresh entropy that is automatically fetched from the
    /// entropy source and updates the working state parameters.
    ///
    /// @param additional_input Pointer to the additional input for DRBG.
    /// @return Result of the DRBG reseed operation.
    fn drbg_reseed(additional_input: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Instantiates the DRBG system.
    ///
    /// Initializes DRBG and the DRBG context. Gets the required entropy input from
    /// the user through the `entropy` parameter. Calling this function breaks FIPS
    /// compliance until the DRBG is uninstantiated.
    ///
    /// The entropy input must be exactly 384 bits long (48 bytes). The
    /// personalization string must not be longer than the entropy input, and may be
    /// empty. If the string is word aligned and the size is a multiple of the word
    /// length (32-bit), it is handled using SCA hardened memory operations. If not,
    /// a non SCA hardened fallback is used.
    ///
    /// @param entropy Pointer to the user defined entropy value.
    /// @param personalization_string Pointer to personalization bitstring.
    /// @return Result of the DRBG manual instantiation.
    fn drbg_manual_instantiate(entropy: &[u8], perso_string: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Reseeds the DRBG with fresh entropy.
    ///
    /// Reseeds the DRBG with entropy input from the user through the `entropy`
    /// parameter and updates the working state parameters. Calling this function
    /// breaks FIPS compliance until the DRBG is uninstantiated.
    ///
    /// @param entropy Pointer to the user defined entropy value.
    /// @param additional_input Pointer to the additional input for DRBG.
    /// @return Result of the manual DRBG reseed operation.
    fn drbg_manual_reseed(entropy: &[u8], additional_input: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// DRBG function for generating random bits.
    ///
    /// This function checks the hardware flags for FIPS compatibility of the
    /// generated bits, so it will fail after `otcrypto_drbg_manual_instantiate` or
    /// `otcrypto_drbg_manual_reseed`.
    ///
    /// The caller should allocate space for the `drbg_output` buffer and set the
    /// length of expected output in the `len` field.
    ///
    /// The output is generated in 16-byte blocks; if `drbg_output->len` is not a
    /// multiple of 4, some output from the hardware will be discarded. This detail
    /// may be important for known-answer tests.
    ///
    /// @param additional_input Pointer to the additional data.
    /// @param[out] drbg_output Pointer to the generated pseudo random bits.
    /// @return Result of the DRBG generate operation.
    fn drbg_generate(additional_input: &[u8], drbg_output: &mut [u8]) -> CryptoResult {
        unimplemented!();
    }
    /// DRBG function for generating random bits.
    ///
    /// This function does NOT check the hardware flags for FIPS compatibility of the
    /// generated bits, so it may be called after `otcrypto_drbg_manual_instantiate`
    /// or `otcrypto_drbg_manual_reseed`.
    ///
    /// The caller should allocate space for the `drbg_output` buffer and set the
    /// length of expected output in the `len` field.
    ///
    /// The output is generated in 16-byte blocks; if `drbg_output->len` is not a
    /// multiple of 4, some output from the hardware will be discarded. This detail
    /// may be important for known-answer tests.
    ///
    /// @param additional_input Pointer to the additional data.
    /// @param[out] drbg_output Pointer to the generated pseudo random bits.
    /// @return Result of the DRBG generate operation.
    fn drbg_manual_generate(additional_input: &[u8], drbg_output: &mut [u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Uninstantiates DRBG and clears the context.
    ///
    /// @return Result of the DRBG uninstantiate operation.
    fn drbg_uninstantiate() -> CryptoResult {
        unimplemented!();
    }
    /// Generates a key pair for ECDSA with curve P-256.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. For a hardware-backed key, use the private key handle returned by
    /// `otcrypto_hw_backed_key`. Otherwise, the mode should indicate ECDSA with
    /// P-256 and the keyblob should be 80 bytes. The value in the `checksum` field
    /// of the blinded key struct will be populated by the key generation function.
    ///
    /// @param[out] private_key Pointer to the blinded private key (d) struct.
    /// @param[out] public_key Pointer to the unblinded public key (Q) struct.
    /// @return Result of the ECDSA key generation.
    fn ecdsa_p256_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates an ECDSA signature with curve P-256.
    ///
    /// The message digest must be exactly 256 bits (32 bytes) long, but may use any
    /// hash mode. The caller is responsible for ensuring that the security
    /// strength of the hash function is at least equal to the security strength of
    /// the curve, but in some cases it may be truncated. See FIPS 186-5 for
    /// details.
    ///
    /// @param private_key Pointer to the blinded private key (d) struct.
    /// @param message_digest Message digest to be signed (pre-hashed).
    /// @param[out] signature Pointer to the signature struct with (r,s) values.
    /// @return Result of the ECDSA signature generation.
    fn ecdsa_p256_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates an ECDSA signature with curve P-256 and verifies the signature
    /// before releasing it to mitigate fault injection attacks.
    ///
    /// The message digest must be exactly 256 bits (32 bytes) long, but may use any
    /// hash mode. The caller is responsible for ensuring that the security
    /// strength of the hash function is at least equal to the security strength of
    /// the curve, but in some cases it may be truncated. See FIPS 186-5 for
    /// details.
    ///
    /// @param private_key Pointer to the blinded private key (d) struct.
    /// @param public_key Pointer to the unblinded public key (Q) struct.
    /// @param message_digest Message digest to be signed (pre-hashed).
    /// @param[out] signature Pointer to the signature struct with (r,s) values.
    /// @return Result of the ECDSA signature generation.
    fn ecdsa_p256_sign_verify(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Verifies an ECDSA/P-256 signature.
    ///
    /// The message digest must be exactly 256 bits (32 bytes) long, but may use any
    /// hash mode. The caller is responsible for ensuring that the security
    /// strength of the hash function is at least equal to the security strength of
    /// the curve, but in some cases it may be truncated. See FIPS 186-5 for
    /// details.
    ///
    /// The caller must check the `verification_result` parameter, NOT only the
    /// returned status code, to know if the signature passed verification. The
    /// status code, as for other operations, only indicates whether errors were
    /// encountered, and may return OK even when the signature is invalid.
    ///
    /// @param public_key Pointer to the unblinded public key (Q) struct.
    /// @param message_digest Message digest to be verified (pre-hashed).
    /// @param signature Pointer to the signature to be verified.
    /// @param[out] verification_result Whether the signature passed verification.
    /// @return Result of the ECDSA verification operation.
    fn ecdsa_p256_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates a key pair for ECDH with curve P-256.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. For a hardware-backed key, use the private key handle returned by
    /// `otcrypto_hw_backed_key`. Otherwise, the mode should indicate ECDH with
    /// P-256 and the keyblob should be 80 bytes. The value in the `checksum` field
    /// of the blinded key struct will be populated by the key generation function.
    ///
    /// @param[out] private_key Pointer to the blinded private key (d) struct.
    /// @param[out] public_key Pointer to the unblinded public key (Q) struct.
    /// @return Result of the ECDH key generation.
    fn ecdh_p256_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Elliptic Curve Diffie Hellman shared secret generation with curve P-256.
    ///
    /// @param private_key Pointer to the blinded private key (d) struct.
    /// @param public_key Pointer to the unblinded public key (Q) struct.
    /// @param[out] shared_secret Pointer to generated blinded shared key struct.
    /// @return Result of ECDH shared secret generation.
    fn ecdh_p256(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        shared_secret: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates a key pair for ECDSA with curve P-384.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. For a hardware-backed key, use the private key handle returned by
    /// `otcrypto_hw_backed_key`. Otherwise, the mode should indicate ECDSA with
    /// P-384 and the keyblob should be 112 bytes. The value in the `checksum` field
    /// of the blinded key struct will be populated by the key generation function.
    ///
    /// @param[out] private_key Pointer to the blinded private key (d) struct.
    /// @param[out] public_key Pointer to the unblinded public key (Q) struct.
    /// @return Result of the ECDSA key generation.
    fn ecdsa_p384_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates an ECDSA signature with curve P-384.
    ///
    /// The message digest must be exactly 384 bits (48 bytes) long, but may use any
    /// hash mode. The caller is responsible for ensuring that the security
    /// strength of the hash function is at least equal to the security strength of
    /// the curve, but in some cases it may be truncated. See FIPS 186-5 for
    /// details.
    ///
    /// @param private_key Pointer to the blinded private key (d) struct.
    /// @param message_digest Message digest to be signed (pre-hashed).
    /// @param[out] signature Pointer to the signature struct with (r,s) values.
    /// @return Result of the ECDSA signature generation.
    fn ecdsa_p384_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates an ECDSA signature with curve P-384 and verifies the signature
    /// before releasing it to mitigate fault injection attacks.
    ///
    /// The message digest must be exactly 384 bits (48 bytes) long, but may use any
    /// hash mode. The caller is responsible for ensuring that the security
    /// strength of the hash function is at least equal to the security strength of
    /// the curve, but in some cases it may be truncated. See FIPS 186-5 for
    /// details.
    ///
    /// @param private_key Pointer to the blinded private key (d) struct.
    /// @param public_key Pointer to the unblinded public key (Q) struct.
    /// @param message_digest Message digest to be signed (pre-hashed).
    /// @param[out] signature Pointer to the signature struct with (r,s) values.
    /// @return Result of the ECDSA signature generation.
    fn ecdsa_p384_sign_verify(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Verifies an ECDSA/P-384 signature.
    ///
    /// The message digest must be exactly 384 bits (48 bytes) long, but may use any
    /// hash mode. The caller is responsible for ensuring that the security
    /// strength of the hash function is at least equal to the security strength of
    /// the curve, but in some cases it may be truncated. See FIPS 186-5 for
    /// details.
    ///
    /// The caller must check the `verification_result` parameter, NOT only the
    /// returned status code, to know if the signature passed verification. The
    /// status code, as for other operations, only indicates whether errors were
    /// encountered, and may return OK even when the signature is invalid.
    ///
    /// @param public_key Pointer to the unblinded public key (Q) struct.
    /// @param message_digest Message digest to be verified (pre-hashed).
    /// @param signature Pointer to the signature to be verified.
    /// @param[out] verification_result Whether the signature passed verification.
    /// @return Result of the ECDSA verification operation.
    fn ecdsa_p384_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates a key pair for ECDH with curve P-384.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. For a hardware-backed key, use the private key handle returned by
    /// `otcrypto_hw_backed_key`. Otherwise, the mode should indicate ECDH with
    /// P-384 and the keyblob should be 112 bytes. The value in the `checksum` field
    /// of the blinded key struct will be populated by the key generation function.
    ///
    /// @param[out] private_key Pointer to the blinded private key (d) struct.
    /// @param[out] public_key Pointer to the unblinded public key (Q) struct.
    /// @return Result of the ECDH key generation.
    fn ecdh_p384_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Elliptic Curve Diffie Hellman shared secret generation with curve P-384.
    ///
    /// @param private_key Pointer to the blinded private key (d) struct.
    /// @param public_key Pointer to the unblinded public key (Q) struct.
    /// @param[out] shared_secret Pointer to generated blinded shared key struct.
    /// @return Result of ECDH shared secret generation.
    fn ecdh_p384(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        shared_secret: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates a key pair for Ed25519.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. For a hardware-backed key, use the private key handle returned by
    /// `otcrypto_hw_backed_key`. Otherwise, the mode should indicate Ed25519 and the
    /// keyblob should be 80 bytes. The value in the `checksum` field of the blinded
    /// key struct will be populated by the key generation function.
    ///
    /// @param[out] private_key Pointer to the blinded private key struct.
    /// @param[out] public_key Pointer to the unblinded public key struct.
    /// @return Result of the Ed25519 key generation.
    fn ed25519_keygen(private_key: &mut BlindedKey, public_key: &mut UnblindedKey) -> CryptoResult {
        unimplemented!();
    }
    /// Generates an Ed25519 digital signature.
    ///
    /// @param private_key Pointer to the blinded private key struct.
    /// @param input_message Input message to be signed.
    /// @param sign_mode EdDSA signature hashing mode.
    /// @param[out] signature Pointer to the EdDSA signature with (r,s) values.
    /// @return Result of the Ed25519 signature generation.
    fn ed25519_sign(
        private_key: &BlindedKey,
        input_message: &[u8],
        sign_mode: EddsaSignMode,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Verifies an Ed25519 signature.
    ///
    /// The caller must check the `verification_result` parameter, NOT only the
    /// returned status code, to know if the signature passed verification. The
    /// status code, as for other operations, only indicates whether errors were
    /// encountered, and may return OK even when the signature is invalid.
    ///
    /// @param public_key Pointer to the unblinded public key struct.
    /// @param input_message Input message to be signed for verification.
    /// @param sign_mode EdDSA signature hashing mode.
    /// @param signature Pointer to the signature to be verified.
    /// @param[out] verification_result Whether the signature passed verification.
    /// @return Result of the Ed25519 verification operation.
    fn ed25519_verify(
        public_key: &UnblindedKey,
        input_message: &[u8],
        sign_mode: EddsaSignMode,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs HKDF (IETF RFC 5869) in one shot, both expand and extract stages.
    ///
    /// The hash mode for the underlying HMAC is determined by the mode of the input
    /// key material, e.g. the key mode `kOtcryptoKeyModeHmacSha256` results in HMAC
    /// with SHA-256.
    ///
    /// The caller should allocate and partially populate the `okm` blinded key
    /// struct, including populating the key configuration and allocating space for
    /// the keyblob. The configuration may not indicate a hardware-backed key and
    /// must indicate a symmetric mode. The allocated keyblob length for the output
    /// key should be twice the unmasked key length indicated in its key
    /// configuration, rounded up to the nearest 32-bit word. This unmasked key
    /// length must not be longer than 255*<length of digest for the chosen hash
    /// mode>, as per the RFC. The value in the `checksum` field of the blinded key
    /// struct will be populated by the key derivation function.
    ///
    /// @param ikm Blinded input key material.
    /// @param salt Salt value (optional, may be empty).
    /// @param info Context-specific string (optional, may be empty).
    /// @param[out] okm Blinded output keying material.
    /// @return Result of the key derivation operation.
    fn hkdf(ikm: &BlindedKey, salt: &[u8], info: &[u8], okm: &mut BlindedKey) -> CryptoResult {
        unimplemented!();
    }
    fn hkdf_extract(ikm: &BlindedKey, salt: &[u8], prk: &mut BlindedKey) -> CryptoResult {
        unimplemented!();
    }
    fn hkdf_expand(prk: &BlindedKey, info: &[u8], okm: &mut BlindedKey) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA2-256 hash computation.
    ///
    /// The caller should allocate space for the `digest` buffer and set the `len`
    /// fields. If the length does not match the mode, an error message will be
    /// returned. The `mode` field will be set by this function.
    ///
    /// @param message Input message to be hashed.
    /// @param[out] digest Output digest after hashing the input message.
    /// @return OK or error.
    fn sha2_256(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA2-384 hash computation.
    ///
    /// The caller should allocate space for the `digest` buffer and set the `len`
    /// fields. If the length does not match the mode, an error message will be
    /// returned. The `mode` field will be set by this function.
    ///
    /// @param message Input message to be hashed.
    /// @param[out] digest Output digest after hashing the input message.
    /// @return OK or error.
    fn sha2_384(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA2-512 hash computation.
    ///
    /// The caller should allocate space for the `digest` buffer and set the `len`
    /// field. If the length does not match the mode, an error message will be
    /// returned. The `mode` field will be set by this function.
    ///
    /// @param message Input message to be hashed.
    /// @param[out] digest Output digest after hashing the input message.
    /// @return OK or error.
    fn sha2_512(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// Start a streaming SHA2 operation.
    ///
    /// @param hash_mode Desired mode (must be a SHA-2 mode).
    /// @param[out] ctx Initialized context object.
    /// @return OK or error.
    fn sha2_init(hash_mode: HashMode, ctx: &mut Sha2Context) -> CryptoResult {
        unimplemented!();
    }
    /// Add more data to a streaming SHA2 operation and update the context.
    ///
    /// @param ctx Initialized context object (updated in place).
    /// @param message Input message data.
    /// @return OK or error.
    fn sha2_update(ctx: &mut Sha2Context, message: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Finish a streaming SHA2 operation.
    ///
    /// The caller should allocate space for the `digest` buffer and set the `len`
    /// field. If the length does not match the context, an error message will be
    /// returned. The `mode` field will be inferred from the length and set by this
    /// function.
    ///
    /// The context data should not be used after this operation.
    ///
    /// @param ctx Initialized context object.
    /// @param[out] digest Resulting digest.
    /// @return OK or error.
    fn sha2_final(ctx: &mut Sha2Context, digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the HMAC function on the input data.
    ///
    /// This function computes the HMAC function on the `input_message` using the
    /// `key` and returns a `tag`. The key should be at least as long as the digest
    /// for the chosen hash function. The hash function is determined by the key
    /// mode. Only SHA-2 hash functions are supported. Other modes (e.g. SHA-3) are
    /// not supported and will result in errors.
    ///
    /// The caller should allocate the following amount of space for the `tag`
    /// buffer, depending on which hash algorithm is used:
    ///
    /// SHA-256: 32 bytes
    /// SHA-384: 48 bytes
    /// SHA-512: 64 bytes
    ///
    /// The caller should also set the `len` field of `tag` to the equivalent number
    /// of 32-bit words (e.g. 8 for SHA-256).
    ///
    /// @param key Pointer to the blinded key struct with key shares.
    /// @param input_message Input message to be hashed.
    /// @param[out] tag Output authentication tag.
    /// @return The result of the HMAC operation.
    fn hmac(key: &BlindedKey, input_message: &[u8], tag: &mut [u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the INIT operation for HMAC.
    ///
    /// Initializes the HMAC context. The key should be at least as long as the
    /// digest for the chosen hash function. The hash function is determined by the
    /// key mode. Only SHA-2 hash functions are are supported. Other modes (e.g.
    /// SHA-3) are not supported and will result in errors.
    ///
    /// @param[out] ctx Pointer to the generic HMAC context struct.
    /// @param key Pointer to the blinded HMAC key struct.
    /// @param hash_mode Hash function to use.
    /// @return Result of the HMAC init operation.
    fn hmac_init(ctx: &mut HmacContext, key: &BlindedKey) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the UPDATE operation for HMAC.
    ///
    /// The update operation processes the `input_message` using the selected
    /// compression function. The intermediate state is stored in the HMAC context
    /// `ctx`. Any partial data is stored back in the context and combined with the
    /// subsequent bytes.
    ///
    /// #otcrypto_hmac_init should be called before calling this function.
    ///
    /// @param ctx Pointer to the generic HMAC context struct.
    /// @param input_message Input message to be hashed.
    /// @return Result of the HMAC update operation.
    fn hmac_update(ctx: &mut HmacContext, input_message: &[u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the FINAL operation for HMAC.
    ///
    /// The final operation processes the remaining partial blocks, computes the
    /// final authentication code and copies it to the `tag` parameter.
    ///
    /// #otcrypto_hmac_update should be called before calling this function.
    ///
    /// The caller should allocate space for the `tag` buffer, (the length should
    /// match the hash function digest size), and set the length of expected output
    /// in the `len` field of `tag`. If the user-set length and the output length
    /// does not match, an error message will be returned.
    ///
    /// @param ctx Pointer to the generic HMAC context struct.
    /// @param[out] tag Output authentication tag.
    /// @return Result of the HMAC final operation.
    fn hmac_final(ctx: &mut HmacContext, tag: &mut [u8]) -> CryptoResult {
        unimplemented!();
    }
    /// Performs KDF-CTR with HMAC as the PRF, according to NIST SP 800-108r1.
    ///
    /// The caller should allocate and partially populate the `output_key_material`
    /// blinded key struct, including populating the key configuration and
    /// allocating space for the keyblob. The configuration may not indicate a
    /// hardware-backed key and must indicate a symmetric mode. The allocated
    /// keyblob length for the output key should be twice the unmasked key length
    /// indicated in its key configuration, rounded up to the nearest 32-bit word.
    /// The value in the `checksum` field of the blinded key struct will be
    /// populated by the key derivation function.
    ///
    /// @param key_derivation_key Blinded key derivation key.
    /// @param label Label string (optional, may be empty).
    /// @param context Context string (optional, may be empty).
    /// @param[out] output_key_material Blinded output key material.
    /// @return Result of the key derivation operation.
    fn kdf_ctr_hmac(
        key_derivation_key: &BlindedKey,
        label: &[u8],
        context: &[u8],
        output_key_material: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates a new, random symmetric key.
    ///
    /// Use this only for symmetric algorithms (e.g. AES, HMAC, KMAC). Asymmetric
    /// algorithms (e.g. ECDSA, RSA) have their own specialized key-generation
    /// routines. Cannot be used for hardware-backed keys; use
    /// `otcrypto_hw_backed_key` instead to generate these.
    ///
    /// The caller should allocate space for the keyblob and populate the blinded
    /// key struct with the length of the keyblob, the pointer to the keyblob
    /// buffer, and the key configuration. The value in the `checksum` field of
    /// the blinded key struct will be populated by the key generation function.
    /// The keyblob should be twice the length of the unblinded key.  This function
    /// will return an error if the keyblob length does not match expectations based
    /// on the key mode and configuration.
    ///
    /// The keyblob should be twice the length of the key. The caller only needs to
    /// allocate the keyblob, not populate it.
    ///
    /// The personalization string may empty, and may be up to 48 bytes long; any
    /// longer will result in an error. It is passed as an extra seed input to the
    /// DRBG, in addition to the hardware TRNG.
    ///
    /// @param perso_string Optional personalization string to be passed to DRBG.
    /// @param[out] key Destination blinded key struct.
    /// @return The result of the operation.
    fn symmetric_keygen(perso_string: &[u8], key: &mut BlindedKey) -> CryptoResult {
        unimplemented!();
    }
    /// Creates a handle for a hardware-backed key.
    ///
    /// This routine may be used for both symmetric and asymmetric algorithms, since
    /// conceptually it only creates some data that the key manager can use to
    /// generate key material at the time of use. However, not all algorithms are
    /// suitable for hardware-backed keys (for example, RSA is not suitable), so
    /// some choices of algorithm may result in errors.
    ///
    /// The caller should partially populate the blinded key struct; they should set
    /// the full key configuration and the keyblob length (always 32 bytes), and
    /// then allocate 32 bytes of space for the keyblob and set the keyblob pointer.
    ///
    /// This function will populate the `checksum` field and copy the salt/version
    /// data into the keyblob buffer in the specific order that the rest of
    /// cryptolib expects.
    ///
    /// @param version Key version.
    /// @param salt Key salt (diversification data for KDF).
    /// @param[out] key Destination blinded key struct.
    /// @return The result of the operation.
    fn hw_backed_key(version: u32, salt: &u32, key: &mut BlindedKey) -> CryptoResult {
        unimplemented!();
    }
    /// Returns the length that the blinded key will have once wrapped.
    ///
    /// @param config Key configuration.
    /// @param[out] wrapped_num_words Number of 32b words for the wrapped key.
    /// @return Result of the operation.
    fn wrapped_key_len(config: KeyConfig, wrapped_num_words: &mut usize) -> CryptoResult {
        unimplemented!();
    }
    /// Wraps (encrypts) a secret key.
    ///
    /// The key wrap function uses AES-KWP (key wrapping with padding), an
    /// authenticated encryption mode designed for encrypting key material.
    ///
    /// The caller should allocate space for the `wrapped_key` buffer according to
    /// `otcrypto_wrapped_key_len`, and set the length of expected output in the
    /// `len` field of `wrapped_key`. If the user-set length and the output length
    /// do not match, an error message will be returned.
    ///
    /// The blinded key struct to wrap must be 32-bit aligned.
    ///
    /// @param key_to_wrap Blinded key that will be encrypted.
    /// @param key_kek AES-KWP key used to encrypt `key_to_wrap`.
    /// @param[out] wrapped_key Encrypted key data.
    /// @return Result of the wrap operation.
    fn key_wrap(
        key_to_wrap: &BlindedKey,
        key_kek: &BlindedKey,
        wrapped_key: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Unwraps (decrypts) a secret key.
    ///
    /// The key unwrap function uses AES-KWP (key wrapping with padding), an
    /// authenticated encryption mode designed for encrypting key material.
    ///
    /// The caller must allocate space for the keyblob and set the keyblob-length
    /// and keyblob fields in `unwrapped_key` accordingly. If there is not enough
    /// space in the keyblob, this function will return an error. Too much space in
    /// the keyblob is okay; this function will write to the first part of the
    /// keyblob buffer and set the keyblob length field to the correct exact value
    /// for the unwrapped key, at which point it is safe to check the new length and
    /// free the remaining keyblob memory. It is always safe to allocate a keyblob
    /// the same size as the wrapped key; this will always be enough space by
    /// definition.
    ///
    /// The caller does not need to populate the blinded key configuration, since
    /// this information is encrypted along with the key.  However, the caller may
    /// want to check that the configuration matches expectations.
    ///
    /// An OK status from this function does NOT necessarily mean that unwrapping
    /// succeeded; the caller must check both the returned status and the `success`
    /// parameter before reading the unwrapped key.
    ///
    /// @param wrapped_key Encrypted key data.
    /// @param key_kek AES-KWP key used to decrypt `wrapped_key`.
    /// @param[out] success Whether the wrapped key was valid.
    /// @param[out] unwrapped_key Decrypted key data.
    /// @return Result of the aes-kwp unwrap operation.
    fn key_unwrap(
        wrapped_key: &[u8],
        key_kek: &BlindedKey,
        success: &mut HardenedBool,
        unwrapped_key: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Creates a blinded key struct from masked key material.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. The keyblob should be twice the length of the user key.
    /// Hardware-backed and asymmetric (ECC or RSA) keys cannot be imported this
    /// way. For asymmetric keys, use algorithm-specific key construction methods.
    ///
    /// This function will copy the data from the shares into the keyblob; it is
    /// safe to free `key_share0` and `key_share1` after this call.
    ///
    /// @param key_share0 First share of the user provided key.
    /// @param key_share1 Second share of the user provided key.
    /// @param[out] blinded_key Generated blinded key struct.
    /// @return Result of the blinded key import operation.
    fn import_blinded_key(
        key_share0: &[u8],
        key_share1: &[u8],
        blinded_key: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Exports a blinded key to the user provided key buffer, in shares.
    ///
    /// This function will copy data from the keyblob into the shares; after the
    /// call, it is safe to free the blinded key and the data pointed to by
    /// `blinded_key.keyblob`.
    ///
    /// Hardware-backed, non-exportable, and asymmetric (ECC or RSA) keys cannot be
    /// exported this way. For asymmetric keys, use an algorithm-specific funtion.
    ///
    /// @param blinded_key Blinded key struct to be exported.
    /// @param[out] key_share0 First share of the blinded key.
    /// @param[out] key_share1 Second share of the blinded key.
    /// @return Result of the blinded key export operation.
    fn export_blinded_key(
        blinded_key: &BlindedKey,
        key_share0: &mut [u8],
        key_share1: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the KMAC function on the input data.
    ///
    /// This function computes the KMAC on the `input_message` using the `key` and
    /// returns a `tag` of `required_output_len`. The customization string is passed
    /// through `customization_string` parameter. If no customization is desired it
    /// can be be left empty (by settings its `data` to NULL and `length` to 0).
    ///
    /// The caller should set the `key_length` field of `key.config` to the number
    /// of bytes in the key. Only the following key sizes (in bytes) are supported:
    /// [16, 24, 32, 48, 64]. If any other size is given, the function will return
    /// an error.
    ///
    /// The KMAC mode (KMAC-128 or KMAC-256) is inferred from the key mode.
    ///
    /// The caller should allocate enough space in the `tag` buffer to hold
    /// `required_output_len` bytes, rounded up to the nearest word, and then set
    /// the `len` field of `tag` to the word length. If the word length is not long
    /// enough to hold `required_output_len` bytes, then the function will return an
    /// error.
    ///
    /// @param key Pointer to the blinded key struct with key shares.
    /// @param input_message Input message to be hashed.
    /// @param customization_string Customization string.
    /// @param required_output_len Required output length, in bytes.
    /// @param[out] tag Output authentication tag.
    /// @return The result of the KMAC operation.
    fn kmac(
        key: &mut BlindedKey,
        input_message: &[u8],
        customization_string: &[u8],
        required_output_len: usize,
        tag: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs KMAC-KDF as specified in NIST SP 800-108r1.
    ///
    /// KMAC-KDF can use either KMAC128 or KMAC256; which one is determined by the
    /// key mode in the configuration of `key_derivation_key`.
    ///
    /// Because of limitations on the KMAC hardware, labels longer than 32 bytes are
    /// not supported.
    ///
    /// The caller should allocate and partially populate the `output_key_material`
    /// blinded key struct, including populating the key configuration and
    /// allocating space for the keyblob. The configuration may not indicate a
    /// hardware-backed key and must indicate a symmetric mode. The allocated
    /// keyblob length for the output key should be twice the unmasked key length
    /// indicated in its key configuration, rounded up to the nearest 32-bit word.
    /// The value in the `checksum` field of the blinded key struct will be
    /// populated by the key derivation function.
    ///
    /// @param key_derivation_key Blinded key derivation key.
    /// @param kmac_mode Either KMAC128 or KMAC256 as PRF.
    /// @param label Label string (optional, may be empty).
    /// @param context Context string (optional, may be empty).
    /// @param[out] output_key_material Blinded output key material.
    /// @return Result of the key derivation operation.
    fn kmac_kdf(
        key_derivation_key: &mut BlindedKey,
        label: &[u8],
        context: &[u8],
        output_key_material: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Performs the RSA key generation.
    ///
    /// Computes RSA private key (d) and the public key modulus (n).
    ///
    /// The caller should allocate space for the public key and set the `key` and
    /// `key_length` fields accordingly.
    ///
    /// The caller should fully populate the blinded key configuration and allocate
    /// space for the keyblob, setting `config.key_length` and `keyblob_length`
    /// accordingly.
    ///
    /// The value in the `checksum` field of key structs is not checked here and
    /// will be populated by the key generation function.
    ///
    /// @param size RSA size parameter.
    /// @param[out] public_key Pointer to public key struct.
    /// @param[out] private_key Pointer to blinded private key struct.
    /// @return Result of the RSA key generation.
    fn rsa_keygen(
        size: RsaSize,
        public_key: &mut UnblindedKey,
        private_key: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Constructs an RSA public key from the modulus and public exponent.
    ///
    /// The caller should allocate space for the public key and set the `key` and
    /// `key_length` fields accordingly. The public exponent is implicitly fixed
    /// to e=2^16+1.
    ///
    /// @param size RSA size parameter.
    /// @param modulus RSA modulus (n).
    /// @param exponent RSA public exponent (e).
    /// @param[out] public_key Destination public key struct.
    /// @return Result of the RSA key construction.
    fn rsa_public_key_construct(
        size: RsaSize,
        modulus: &[u8],
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Constructs an RSA private key from the modulus and public/private exponents.
    ///
    /// The caller should allocate space for the private key and set the `keyblob`,
    /// `keyblob_length`, and `key_length` fields accordingly.
    ///
    /// @param size RSA size parameter.
    /// @param modulus RSA modulus (n).
    /// @param d_share0 First share of the RSA private exponent d.
    /// @param d_share1 Second share of the RSA private exponent d.
    /// @param[out] public_key Destination public key struct.
    /// @return Result of the RSA key construction.
    fn rsa_private_key_from_exponents(
        size: RsaSize,
        modulus: &[u8],
        d_share0: &[u8],
        d_share1: &[u8],
        private_key: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Constructs an RSA keypair from the public key and one prime cofactor.
    ///
    /// The caller should allocate space for the private key and set the `keyblob`,
    /// `keyblob_length`, and `key_length` fields accordingly. Similarly, the caller
    /// should allocate space for the public key and set the `key` and `key_length`
    /// fields.
    ///
    /// @param size RSA size parameter.
    /// @param modulus RSA modulus (n).
    /// @param cofactor_share0 First share of the prime cofactor (p or q).
    /// @param cofactor_share1 Second share of the prime cofactor (p or q).
    /// @param[out] public_key Destination public key struct.
    /// @param[out] private_key Destination private key struct.
    /// @return Result of the RSA key construction.
    fn rsa_keypair_from_cofactor(
        size: RsaSize,
        modulus: &[u8],
        cofactor_share0: &[u8],
        cofactor_share1: &[u8],
        public_key: &mut UnblindedKey,
        private_key: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Computes the digital signature on the input message data.
    ///
    /// The caller should allocate space for the `signature` buffer
    /// and set the length of expected output in the `len` field of
    /// `signature`. If the user-set length and the output length does not
    /// match, an error message will be returned.
    ///
    /// @param private_key Pointer to blinded private key struct.
    /// @param message_digest Message digest to be signed (pre-hashed).
    /// @param padding_mode Padding scheme to be used for the data.
    /// @param[out] signature Pointer to the generated signature struct.
    /// @return The result of the RSA signature generation.
    fn rsa_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        padding_mode: RsaPadding,
        signature: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    fn rsa_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        padding_mode: RsaPadding,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Encrypts a message with RSA.
    ///
    /// The only padding scheme available is OAEP, where the hash function is a
    /// member of the SHA-2 or SHA-3 family and the mask generation function is
    /// MGF1 with the same hash function.
    ///
    /// OAEP imposes strict limits on the length of the message (see IETF RFC 8017
    /// for details). Specifically, the message is at most k - 2*hLen - 2 bytes
    /// long, where k is the byte-length of the RSA modulus and hLen is the length
    /// of the hash function digest. If the message is too long, this function will
    /// return an error.
    ///
    /// The caller should allocate space for the `ciphertext` buffer and set the
    /// length of expected output in the `len` field of `signature`. The ciphertext
    /// is always the same length as the RSA modulus (so an RSA-2048 ciphertext is
    /// always 2048 bits long). If the length does not match the private key mode,
    /// this function returns an error.
    ///
    /// Note: RSA encryption is included for compatibility with legacy interfaces,
    /// and is typically not recommended for modern applications because it is
    /// slower and more fragile than other encryption methods. Consult an expert
    /// before using RSA encryption.
    ///
    /// @param private_key Pointer to public key struct.
    /// @param hash_mode Hash function to use for OAEP encoding.
    /// @param message Message to encrypt.
    /// @param label Label for OAEP encoding.
    /// @param[out] ciphertext Buffer for the ciphertext.
    /// @return The result of the RSA encryption operation.
    fn rsa_encrypt(
        public_key: &UnblindedKey,
        hash_mode: HashMode,
        message: &[u8],
        label: &[u8],
        ciphertext: &mut [u8],
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Decrypts a message with RSA.
    ///
    /// The only padding scheme available is OAEP, where the hash function is a
    /// member of the SHA-2 or SHA-3 family and the mask generation function is
    /// MGF1 with the same hash function.
    ///
    /// The caller should allocate space for the `plaintext` buffer and set the
    /// allocated length in the `len` field. The length should be at least as long
    /// as the maximum message length imposed by OAEP; that is, k - 2*hLen - 2 bytes
    /// long, where k is the byte-length of the RSA modulus and hLen is the length
    /// of the hash function digest. If the plaintext buffer is not long enough,
    /// this function will return an error.
    ///
    /// Decryption recovers the original length of the plaintext buffer and will
    /// return its value in `plaintext_bytelen`.
    ///
    /// Note: RSA encryption is included for compatibility with legacy interfaces,
    /// and is typically not recommended for modern applications because it is
    /// slower and more fragile than other encryption methods. Consult an expert
    /// before using RSA encryption.
    ///
    /// @param private_key Pointer to blinded private key struct.
    /// @param hash_mode Hash function to use for OAEP encoding.
    /// @param ciphertext Ciphertext to decrypt.
    /// @param label Label for OAEP encoding.
    /// @param[out] plaintext Buffer for the decrypted message.
    /// @param[out] plaintext_bytelen Recovered byte-length of plaintext.
    /// @return Result of the RSA decryption operation.
    fn rsa_decrypt(
        private_key: &BlindedKey,
        hash_mode: HashMode,
        ciphertext: &[u8],
        label: &[u8],
        plaintext: &mut [u8],
        plaintext_bytelen: &mut usize,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA3-224 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// accordingly. The function will return an error if the length is not 224
    /// bits (= 7 32-bit words). The `digest.mode` field is set by this function and
    /// may be uninitialized.
    ///
    /// @param message Input message.
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn sha3_224(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA3-256 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// accordingly. The function will return an error if the length is not 256
    /// bits (= 8 32-bit words). The `digest.mode` field is set by this function and
    /// may be uninitialized.
    ///
    /// @param message Input message.
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn sha3_256(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA3-384 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// accordingly. The function will return an error if the length is not 384
    /// bits (= 12 32-bit words). The `digest.mode` field is set by this function and
    /// may be uninitialized.
    ///
    /// @param message Input message.
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn sha3_384(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHA3-512 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// accordingly. The function will return an error if the length is not 512
    /// bits (= 16 32-bit words). The `digest.mode` field is set by this function and
    /// may be uninitialized.
    ///
    /// @param message Input message.
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn sha3_512(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHAKE128 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// according to their desired output length. The `digest.mode` field is set by
    /// this function and may be uninitialized.
    ///
    /// @param message Input message.
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn shake128(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot SHAKE256 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// according to their desired output length. The `digest.mode` field is set by
    /// this function and may be uninitialized.
    ///
    /// @param message Input message.
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn shake256(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot cSHAKE128 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// according to their desired output length. The `digest.mode` field is set by
    /// this function and may be uninitialized.
    ///
    /// The function name and customization string parameters are defined in NIST
    /// SP800-185; please refer to that document for guidance on their usage.
    ///
    /// @param message Input message.
    /// @param function_name_string Function name parameter (may be empty).
    /// @param customization_string Customization parameter (may be empty).
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn cshake128(
        message: &[u8],
        function_name_string: &[u8],
        customization_string: &[u8],
        digest: &mut HashDigest,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// One-shot cSHAKE256 hash computation.
    ///
    /// The caller should allocate space for the digest and set `digest.len`
    /// according to their desired output length. The `digest.mode` field is set by
    /// this function and may be uninitialized.
    ///
    /// The function name and customization string parameters are defined in NIST
    /// SP800-185; please refer to that document for guidance on their usage.
    ///
    /// @param message Input message.
    /// @param function_name_string Function name parameter (may be empty).
    /// @param customization_string Customization parameter (may be empty).
    /// @param[out] digest Computed digest.
    /// @return OK or error.
    fn cshake256(
        message: &[u8],
        function_name_string: &[u8],
        customization_string: &[u8],
        digest: &mut HashDigest,
    ) -> CryptoResult {
        unimplemented!();
    }
    /// Generates a key pair for X25519.
    ///
    /// The caller should allocate and partially populate the blinded key struct,
    /// including populating the key configuration and allocating space for the
    /// keyblob. For a hardware-backed key, use the private key handle returned by
    /// `otcrypto_hw_backed_key`. Otherwise, the mode should indicate X25519 and the
    /// keyblob should be 80 bytes. The value in the `checksum` field of the blinded
    /// key struct will be populated by the key generation function.
    ///
    /// @param[out] private_key Pointer to the blinded private key struct.
    /// @param[out] public_key Pointer to the unblinded public key struct.
    /// @return Result of the X25519 key generation.
    fn x25519_keygen(private_key: &mut BlindedKey, public_key: &mut UnblindedKey) -> CryptoResult {
        unimplemented!();
    }
    /// Elliptic-curve Diffie Hellman shared secret generation with Curve25519.
    ///
    /// @param private_key Pointer to blinded private key (u-coordinate).
    /// @param public_key Pointer to the public scalar from the sender.
    /// @param[out] shared_secret Pointer to shared secret key (u-coordinate).
    /// @return Result of the X25519 operation.
    fn x25519(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        shared_secret: &mut BlindedKey,
    ) -> CryptoResult {
        unimplemented!();
    }
}

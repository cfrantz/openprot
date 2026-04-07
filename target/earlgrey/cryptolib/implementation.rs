
use crate::datatypes::*;
use crate::interface::CryptoInterface;
use crate::misc::GetPointer;
use crate::otcrypto::*;

pub struct OtCrypto;
impl CryptoInterface for OtCrypto {
    fn aes_padded_plaintext_length(
        plaintext_len: usize,
        aes_padding: AesPadding,
        padded_len: &mut usize,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_padded_plaintext_length(
                plaintext_len.into(),
                aes_padding.into(),
                padded_len.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn aes(
        key: &mut BlindedKey,
        iv: &mut [u8],
        aes_mode: AesMode,
        aes_operation: AesOperation,
        cipher_input: &[u8],
        aes_padding: AesPadding,
        cipher_output: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes(
                key.as_mut_ptr(),
                otcrypto_word32_buf_t::from(iv),
                aes_mode.into(),
                aes_operation.into(),
                otcrypto_const_byte_buf_t::from(cipher_input),
                aes_padding.into(),
                otcrypto_byte_buf_t::from(cipher_output),
            )
        };
        result.into()
    }
    fn aes_gcm_encrypt(
        key: &mut BlindedKey,
        plaintext: &[u8],
        iv: &[u8],
        aad: &[u8],
        tag_len: AesGcmTagLen,
        ciphertext: &mut [u8],
        auth_tag: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_encrypt(
                key.as_mut_ptr(),
                otcrypto_const_byte_buf_t::from(plaintext),
                otcrypto_const_word32_buf_t::from(iv),
                otcrypto_const_byte_buf_t::from(aad),
                tag_len.into(),
                otcrypto_byte_buf_t::from(ciphertext),
                otcrypto_word32_buf_t::from(auth_tag),
            )
        };
        result.into()
    }
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
        let result = unsafe {
            otcrypto_aes_gcm_decrypt(
                key.as_mut_ptr(),
                otcrypto_const_byte_buf_t::from(ciphertext),
                otcrypto_const_word32_buf_t::from(iv),
                otcrypto_const_byte_buf_t::from(aad),
                tag_len.into(),
                otcrypto_const_word32_buf_t::from(auth_tag),
                otcrypto_byte_buf_t::from(plaintext),
                success.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn aes_gcm_encrypt_init(
        key: &mut BlindedKey,
        iv: &[u8],
        ctx: &mut AesGcmContext,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_encrypt_init(
                key.as_mut_ptr(),
                otcrypto_const_word32_buf_t::from(iv),
                ctx.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn aes_gcm_decrypt_init(
        key: &mut BlindedKey,
        iv: &[u8],
        ctx: &mut AesGcmContext,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_decrypt_init(
                key.as_mut_ptr(),
                otcrypto_const_word32_buf_t::from(iv),
                ctx.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn aes_gcm_update_aad(ctx: &mut AesGcmContext, aad: &[u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_update_aad(ctx.as_mut_ptr(), otcrypto_const_byte_buf_t::from(aad))
        };
        result.into()
    }
    fn aes_gcm_update_encrypted_data(
        ctx: &mut AesGcmContext,
        input: &[u8],
        output: &mut [u8],
        output_bytes_written: &mut usize,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_update_encrypted_data(
                ctx.as_mut_ptr(),
                otcrypto_const_byte_buf_t::from(input),
                otcrypto_byte_buf_t::from(output),
                output_bytes_written.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn aes_gcm_encrypt_final(
        ctx: &mut AesGcmContext,
        tag_len: AesGcmTagLen,
        ciphertext: &mut [u8],
        ciphertext_bytes_written: &mut usize,
        auth_tag: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_encrypt_final(
                ctx.as_mut_ptr(),
                tag_len.into(),
                otcrypto_byte_buf_t::from(ciphertext),
                ciphertext_bytes_written.as_mut_ptr(),
                otcrypto_word32_buf_t::from(auth_tag),
            )
        };
        result.into()
    }
    fn aes_gcm_decrypt_final(
        ctx: &mut AesGcmContext,
        auth_tag: &[u8],
        tag_len: AesGcmTagLen,
        plaintext: &mut [u8],
        plaintext_bytes_written: &mut usize,
        success: &mut HardenedBool,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_aes_gcm_decrypt_final(
                ctx.as_mut_ptr(),
                otcrypto_const_word32_buf_t::from(auth_tag),
                tag_len.into(),
                otcrypto_byte_buf_t::from(plaintext),
                plaintext_bytes_written.as_mut_ptr(),
                success.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn build_info(
        version: &mut u32,
        released: &mut u8,
        build_hash_low: &mut u32,
        build_hash_high: &mut u32,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_build_info(
                version.as_mut_ptr(),
                released.as_mut_ptr(),
                build_hash_low.as_mut_ptr(),
                build_hash_high.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn dice_p256_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        let result = unsafe { dice_p256_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr()) };
        result.into()
    }
    fn dice_p256_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            dice_p256_sign(
                private_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn dice_p256_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
        recovered_r: &mut u32,
    ) -> CryptoResult {
        let result = unsafe {
            dice_p256_verify(
                public_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_word32_buf_t::from(signature),
                recovered_r.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn drbg_instantiate(perso_string: &[u8]) -> CryptoResult {
        let result =
            unsafe { otcrypto_drbg_instantiate(otcrypto_const_byte_buf_t::from(perso_string)) };
        result.into()
    }
    fn drbg_reseed(additional_input: &[u8]) -> CryptoResult {
        let result =
            unsafe { otcrypto_drbg_reseed(otcrypto_const_byte_buf_t::from(additional_input)) };
        result.into()
    }
    fn drbg_manual_instantiate(entropy: &[u8], perso_string: &[u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_drbg_manual_instantiate(
                otcrypto_const_byte_buf_t::from(entropy),
                otcrypto_const_byte_buf_t::from(perso_string),
            )
        };
        result.into()
    }
    fn drbg_manual_reseed(entropy: &[u8], additional_input: &[u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_drbg_manual_reseed(
                otcrypto_const_byte_buf_t::from(entropy),
                otcrypto_const_byte_buf_t::from(additional_input),
            )
        };
        result.into()
    }
    fn drbg_generate(additional_input: &[u8], drbg_output: &mut [u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_drbg_generate(
                otcrypto_const_byte_buf_t::from(additional_input),
                otcrypto_word32_buf_t::from(drbg_output),
            )
        };
        result.into()
    }
    fn drbg_manual_generate(additional_input: &[u8], drbg_output: &mut [u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_drbg_manual_generate(
                otcrypto_const_byte_buf_t::from(additional_input),
                otcrypto_word32_buf_t::from(drbg_output),
            )
        };
        result.into()
    }
    fn drbg_uninstantiate() -> CryptoResult {
        let result = unsafe { otcrypto_drbg_uninstantiate() };
        result.into()
    }
    fn ecdsa_p256_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p256_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr())
        };
        result.into()
    }
    fn ecdsa_p256_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p256_sign(
                private_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn ecdsa_p256_sign_verify(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p256_sign_verify(
                private_key.as_ptr(),
                public_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn ecdsa_p256_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p256_verify(
                public_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_const_word32_buf_t::from(signature),
                verification_result.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn ecdh_p256_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        let result =
            unsafe { otcrypto_ecdh_p256_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr()) };
        result.into()
    }
    fn ecdh_p256(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        shared_secret: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdh_p256(
                private_key.as_ptr(),
                public_key.as_ptr(),
                shared_secret.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn ecdsa_p384_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p384_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr())
        };
        result.into()
    }
    fn ecdsa_p384_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p384_sign(
                private_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn ecdsa_p384_sign_verify(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p384_sign_verify(
                private_key.as_ptr(),
                public_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn ecdsa_p384_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdsa_p384_verify(
                public_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                otcrypto_const_word32_buf_t::from(signature),
                verification_result.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn ecdh_p384_keygen(
        private_key: &mut BlindedKey,
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        let result =
            unsafe { otcrypto_ecdh_p384_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr()) };
        result.into()
    }
    fn ecdh_p384(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        shared_secret: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ecdh_p384(
                private_key.as_ptr(),
                public_key.as_ptr(),
                shared_secret.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn ed25519_keygen(private_key: &mut BlindedKey, public_key: &mut UnblindedKey) -> CryptoResult {
        let result =
            unsafe { otcrypto_ed25519_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr()) };
        result.into()
    }
    fn ed25519_sign(
        private_key: &BlindedKey,
        input_message: &[u8],
        sign_mode: EddsaSignMode,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ed25519_sign(
                private_key.as_ptr(),
                otcrypto_const_byte_buf_t::from(input_message),
                sign_mode.into(),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn ed25519_verify(
        public_key: &UnblindedKey,
        input_message: &[u8],
        sign_mode: EddsaSignMode,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_ed25519_verify(
                public_key.as_ptr(),
                otcrypto_const_byte_buf_t::from(input_message),
                sign_mode.into(),
                otcrypto_const_word32_buf_t::from(signature),
                verification_result.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn hkdf(ikm: &BlindedKey, salt: &[u8], info: &[u8], okm: &mut BlindedKey) -> CryptoResult {
        let result = unsafe {
            otcrypto_hkdf(
                ikm.as_ptr(),
                otcrypto_const_byte_buf_t::from(salt),
                otcrypto_const_byte_buf_t::from(info),
                okm.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn hkdf_extract(ikm: &BlindedKey, salt: &[u8], prk: &mut BlindedKey) -> CryptoResult {
        let result = unsafe {
            otcrypto_hkdf_extract(
                ikm.as_ptr(),
                otcrypto_const_byte_buf_t::from(salt),
                prk.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn hkdf_expand(prk: &BlindedKey, info: &[u8], okm: &mut BlindedKey) -> CryptoResult {
        let result = unsafe {
            otcrypto_hkdf_expand(
                prk.as_ptr(),
                otcrypto_const_byte_buf_t::from(info),
                okm.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn sha2_256(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha2_256(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn sha2_384(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha2_384(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn sha2_512(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha2_512(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn sha2_init(hash_mode: HashMode, ctx: &mut Sha2Context) -> CryptoResult {
        let result = unsafe { otcrypto_sha2_init(hash_mode.into(), ctx.as_mut_ptr()) };
        result.into()
    }
    fn sha2_update(ctx: &mut Sha2Context, message: &[u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_sha2_update(ctx.as_mut_ptr(), otcrypto_const_byte_buf_t::from(message))
        };
        result.into()
    }
    fn sha2_final(ctx: &mut Sha2Context, digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result = unsafe { otcrypto_sha2_final(ctx.as_mut_ptr(), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn hmac(key: &BlindedKey, input_message: &[u8], tag: &mut [u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_hmac(
                key.as_ptr(),
                otcrypto_const_byte_buf_t::from(input_message),
                otcrypto_word32_buf_t::from(tag),
            )
        };
        result.into()
    }
    fn hmac_init(ctx: &mut HmacContext, key: &BlindedKey) -> CryptoResult {
        let result = unsafe { otcrypto_hmac_init(ctx.as_mut_ptr(), key.as_ptr()) };
        result.into()
    }
    fn hmac_update(ctx: &mut HmacContext, input_message: &[u8]) -> CryptoResult {
        let result = unsafe {
            otcrypto_hmac_update(
                ctx.as_mut_ptr(),
                otcrypto_const_byte_buf_t::from(input_message),
            )
        };
        result.into()
    }
    fn hmac_final(ctx: &mut HmacContext, tag: &mut [u8]) -> CryptoResult {
        let result =
            unsafe { otcrypto_hmac_final(ctx.as_mut_ptr(), otcrypto_word32_buf_t::from(tag)) };
        result.into()
    }
    fn kdf_ctr_hmac(
        key_derivation_key: &BlindedKey,
        label: &[u8],
        context: &[u8],
        output_key_material: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_kdf_ctr_hmac(
                key_derivation_key.as_ptr(),
                otcrypto_const_byte_buf_t::from(label),
                otcrypto_const_byte_buf_t::from(context),
                output_key_material.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn symmetric_keygen(perso_string: &[u8], key: &mut BlindedKey) -> CryptoResult {
        let result = unsafe {
            otcrypto_symmetric_keygen(
                otcrypto_const_byte_buf_t::from(perso_string),
                key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn hw_backed_key(version: u32, salt: &u32, key: &mut BlindedKey) -> CryptoResult {
        let result =
            unsafe { otcrypto_hw_backed_key(version.into(), salt.as_ptr(), key.as_mut_ptr()) };
        result.into()
    }
    fn wrapped_key_len(config: KeyConfig, wrapped_num_words: &mut usize) -> CryptoResult {
        let result =
            unsafe { otcrypto_wrapped_key_len(config.into(), wrapped_num_words.as_mut_ptr()) };
        result.into()
    }
    fn key_wrap(
        key_to_wrap: &BlindedKey,
        key_kek: &BlindedKey,
        wrapped_key: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_key_wrap(
                key_to_wrap.as_ptr(),
                key_kek.as_ptr(),
                otcrypto_word32_buf_t::from(wrapped_key),
            )
        };
        result.into()
    }
    fn key_unwrap(
        wrapped_key: &[u8],
        key_kek: &BlindedKey,
        success: &mut HardenedBool,
        unwrapped_key: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_key_unwrap(
                otcrypto_const_word32_buf_t::from(wrapped_key),
                key_kek.as_ptr(),
                success.as_mut_ptr(),
                unwrapped_key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn import_blinded_key(
        key_share0: &[u8],
        key_share1: &[u8],
        blinded_key: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_import_blinded_key(
                otcrypto_const_word32_buf_t::from(key_share0),
                otcrypto_const_word32_buf_t::from(key_share1),
                blinded_key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn export_blinded_key(
        blinded_key: &BlindedKey,
        key_share0: &mut [u8],
        key_share1: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_export_blinded_key(
                blinded_key.as_ptr(),
                otcrypto_word32_buf_t::from(key_share0),
                otcrypto_word32_buf_t::from(key_share1),
            )
        };
        result.into()
    }
    fn kmac(
        key: &mut BlindedKey,
        input_message: &[u8],
        customization_string: &[u8],
        required_output_len: usize,
        tag: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_kmac(
                key.as_mut_ptr(),
                otcrypto_const_byte_buf_t::from(input_message),
                otcrypto_const_byte_buf_t::from(customization_string),
                required_output_len.into(),
                otcrypto_word32_buf_t::from(tag),
            )
        };
        result.into()
    }
    fn kmac_kdf(
        key_derivation_key: &mut BlindedKey,
        label: &[u8],
        context: &[u8],
        output_key_material: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_kmac_kdf(
                key_derivation_key.as_mut_ptr(),
                otcrypto_const_byte_buf_t::from(label),
                otcrypto_const_byte_buf_t::from(context),
                output_key_material.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn rsa_keygen(
        size: RsaSize,
        public_key: &mut UnblindedKey,
        private_key: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_keygen(
                size.into(),
                public_key.as_mut_ptr(),
                private_key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn rsa_public_key_construct(
        size: RsaSize,
        modulus: &[u8],
        public_key: &mut UnblindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_public_key_construct(
                size.into(),
                otcrypto_const_word32_buf_t::from(modulus),
                public_key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn rsa_private_key_from_exponents(
        size: RsaSize,
        modulus: &[u8],
        d_share0: &[u8],
        d_share1: &[u8],
        private_key: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_private_key_from_exponents(
                size.into(),
                otcrypto_const_word32_buf_t::from(modulus),
                otcrypto_const_word32_buf_t::from(d_share0),
                otcrypto_const_word32_buf_t::from(d_share1),
                private_key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn rsa_keypair_from_cofactor(
        size: RsaSize,
        modulus: &[u8],
        cofactor_share0: &[u8],
        cofactor_share1: &[u8],
        public_key: &mut UnblindedKey,
        private_key: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_keypair_from_cofactor(
                size.into(),
                otcrypto_const_word32_buf_t::from(modulus),
                otcrypto_const_word32_buf_t::from(cofactor_share0),
                otcrypto_const_word32_buf_t::from(cofactor_share1),
                public_key.as_mut_ptr(),
                private_key.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn rsa_sign(
        private_key: &BlindedKey,
        message_digest: &HashDigest,
        padding_mode: RsaPadding,
        signature: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_sign(
                private_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                padding_mode.into(),
                otcrypto_word32_buf_t::from(signature),
            )
        };
        result.into()
    }
    fn rsa_verify(
        public_key: &UnblindedKey,
        message_digest: &HashDigest,
        padding_mode: RsaPadding,
        signature: &[u8],
        verification_result: &mut HardenedBool,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_verify(
                public_key.as_ptr(),
                otcrypto_hash_digest::from(message_digest),
                padding_mode.into(),
                otcrypto_const_word32_buf_t::from(signature),
                verification_result.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn rsa_encrypt(
        public_key: &UnblindedKey,
        hash_mode: HashMode,
        message: &[u8],
        label: &[u8],
        ciphertext: &mut [u8],
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_encrypt(
                public_key.as_ptr(),
                hash_mode.into(),
                otcrypto_const_byte_buf_t::from(message),
                otcrypto_const_byte_buf_t::from(label),
                otcrypto_word32_buf_t::from(ciphertext),
            )
        };
        result.into()
    }
    fn rsa_decrypt(
        private_key: &BlindedKey,
        hash_mode: HashMode,
        ciphertext: &[u8],
        label: &[u8],
        plaintext: &mut [u8],
        plaintext_bytelen: &mut usize,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_rsa_decrypt(
                private_key.as_ptr(),
                hash_mode.into(),
                otcrypto_const_word32_buf_t::from(ciphertext),
                otcrypto_const_byte_buf_t::from(label),
                otcrypto_byte_buf_t::from(plaintext),
                plaintext_bytelen.as_mut_ptr(),
            )
        };
        result.into()
    }
    fn sha3_224(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha3_224(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn sha3_256(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha3_256(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn sha3_384(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha3_384(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn sha3_512(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_sha3_512(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn shake128(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_shake128(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn shake256(message: &[u8], digest: &mut HashDigest) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result =
            unsafe { otcrypto_shake256(otcrypto_const_byte_buf_t::from(message), &mut digest_) };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn cshake128(
        message: &[u8],
        function_name_string: &[u8],
        customization_string: &[u8],
        digest: &mut HashDigest,
    ) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result = unsafe {
            otcrypto_cshake128(
                otcrypto_const_byte_buf_t::from(message),
                otcrypto_const_byte_buf_t::from(function_name_string),
                otcrypto_const_byte_buf_t::from(customization_string),
                &mut digest_,
            )
        };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn cshake256(
        message: &[u8],
        function_name_string: &[u8],
        customization_string: &[u8],
        digest: &mut HashDigest,
    ) -> CryptoResult {
        let mut digest_ = otcrypto_hash_digest::from(&mut *digest);
        let result = unsafe {
            otcrypto_cshake256(
                otcrypto_const_byte_buf_t::from(message),
                otcrypto_const_byte_buf_t::from(function_name_string),
                otcrypto_const_byte_buf_t::from(customization_string),
                &mut digest_,
            )
        };
        digest.mode = HashMode(digest_.mode);
        result.into()
    }
    fn x25519_keygen(private_key: &mut BlindedKey, public_key: &mut UnblindedKey) -> CryptoResult {
        let result =
            unsafe { otcrypto_x25519_keygen(private_key.as_mut_ptr(), public_key.as_mut_ptr()) };
        result.into()
    }
    fn x25519(
        private_key: &BlindedKey,
        public_key: &UnblindedKey,
        shared_secret: &mut BlindedKey,
    ) -> CryptoResult {
        let result = unsafe {
            otcrypto_x25519(
                private_key.as_ptr(),
                public_key.as_ptr(),
                shared_secret.as_mut_ptr(),
            )
        };
        result.into()
    }
}

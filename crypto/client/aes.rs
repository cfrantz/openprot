use crate::backend::CryptoClient;
use crate::util;
use crypto_common::symmetric_key::{Aes128Key, Aes192Key, Aes256Key};
use crypto_common::CipherMode;
use crypto_common::Opcode;
use crypto_traits::symmetric;
use crypto_traits::Algorithm;

use zerocopy::IntoBytes;

macro_rules! aes_impl {
    ($algo:path, $key:path) => {
        impl Algorithm<CryptoClient> for $algo {}
        impl symmetric::AlgoParams<CryptoClient, $algo> for CipherMode {
            type Key = $key;
        }

        impl symmetric::SymmetricOp<$key, $algo, CipherMode> for CryptoClient {
            fn encrypt(
                &self,
                mode: &CipherMode,
                key: &$key,
                iv: &mut [u8],
                plaintext: &[u8],
                ciphertext: &mut [u8],
            ) -> Result<(), Self::Error> {
                util::symmetric::encrypt_decrypt(
                    self,
                    Opcode::AES_ENCRYPT,
                    mode,
                    key.as_bytes(),
                    iv,
                    plaintext,
                    ciphertext,
                )
            }

            fn decrypt(
                &self,
                mode: &CipherMode,
                key: &$key,
                iv: &mut [u8],
                ciphertext: &[u8],
                plaintext: &mut [u8],
            ) -> Result<(), Self::Error> {
                util::symmetric::encrypt_decrypt(
                    self,
                    Opcode::AES_DECRYPT,
                    mode,
                    key.as_bytes(),
                    iv,
                    ciphertext,
                    plaintext,
                )
            }
        }
    };
}

aes_impl!(symmetric::Aes128, Aes128Key);
aes_impl!(symmetric::Aes192, Aes192Key);
aes_impl!(symmetric::Aes256, Aes256Key);

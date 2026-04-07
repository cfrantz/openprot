use crate::backend::Backend;
use crate::error::ErrorType;
use crate::Algorithm;

#[derive(Clone, Copy)]
pub enum AesKeySize {
    Aes128 = 16,
    Aes192 = 24,
    Aes256 = 32,
}

#[derive(Clone, Copy)]
pub enum AesMode {
    Ecb,
    Cbc,
    Ctr,
    Gcm,
}

/// The AES algorithm instance, parameterized by key size and mode.
pub struct Aes {
    pub key_size: AesKeySize,
    pub mode: AesMode,
}

impl<B: Backend + ?Sized> Algorithm<B> for Aes {}

pub trait AesCipher: ErrorType
where
    Self: Backend,
{
    type Key;
    type Iv;

    /// One-shot encryption/decryption
    fn encrypt(
        &self,
        algo: &Aes,
        key: &Self::Key,
        iv: Option<&Self::Iv>,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<usize, Self::Error>;
    fn decrypt(
        &self,
        algo: &Aes,
        key: &Self::Key,
        iv: Option<&Self::Iv>,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<usize, Self::Error>;

    /// One-shot Authenticated Encryption (GCM)
    fn seal(
        &self,
        algo: &Aes,
        key: &Self::Key,
        iv: &[u8],
        aad: &[u8],
        tag_len: usize,
        input: &[u8],
        output: &mut [u8],
        tag: &mut [u8],
    ) -> Result<(), Self::Error>;
    fn open(
        &self,
        algo: &Aes,
        key: &Self::Key,
        iv: &[u8],
        aad: &[u8],
        tag: &[u8],
        input: &[u8],
        output: &mut [u8],
    ) -> Result<bool, Self::Error>;
}

use crate::backend::Backend;
use crate::error::ErrorType;
use crate::Algorithm;

/// Common algorithms for asymmetric cryptography.
/// For each algorithm supported by your backend, you should implement
/// Algorithm<YourBackend> on the corresponding struct.
#[derive(Clone, Copy)]
pub struct Aes128;
#[derive(Clone, Copy)]
pub struct Aes192;
#[derive(Clone, Copy)]
pub struct Aes256;

/// The algorithm params represent specific configurations of the algorithm for the backend.
pub trait AlgoParams<B: Backend + ?Sized, A: Algorithm<B>> {
    type Key;
}

pub trait SoftwareKey<B: Backend + ?Sized, A: Algorithm<B>>: AlgoParams<B, A> {
    fn new() -> Self;
}
pub trait HardwareKey<B: Backend + ?Sized, A: Algorithm<B>>: AlgoParams<B, A> {
    type Salt;
    fn new(salt: Self::Salt) -> Self;
}

pub trait SymmetricOp<K: ?Sized, A, P>: ErrorType
where
    Self: Backend,
    A: Algorithm<Self>,
    P: AlgoParams<Self, A, Key = K>,
{
    fn encrypt(
        &self,
        mode: &P,
        key: &K,
        iv: &mut [u8],
        plaintext: &[u8],
        ciphertext: &mut [u8],
    ) -> Result<(), Self::Error>;

    fn decrypt(
        &self,
        mode: &P,
        key: &K,
        iv: &mut [u8],
        ciphertext: &[u8],
        plaintext: &mut [u8],
    ) -> Result<(), Self::Error>;
}

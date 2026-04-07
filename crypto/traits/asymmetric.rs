use crate::backend::Backend;
use crate::digest::Digest;
use crate::error::ErrorType;

/// The algorithm trait represents an algorithm that the backend can perform.
pub trait Algorithm<B: Backend + ?Sized> {}

/// Common algorithms for asymmetric cryptography.
/// For each algorithm supported by your backend, you should implement
/// Algorithm<YourBackend> on the corresponding struct.
#[derive(Clone, Copy)]
pub struct EcdsaP256;
#[derive(Clone, Copy)]
pub struct EcdsaP384;
#[derive(Clone, Copy)]
pub struct Ed25519;
#[derive(Clone, Copy)]
pub struct Rsa2048;
#[derive(Clone, Copy)]
pub struct Rsa3072;
#[derive(Clone, Copy)]
pub struct Rsa4096;

/// The algorithm params represent specific configurations of the algorithm for the backend.
pub trait AlgoParams<B: Backend + ?Sized, A: Algorithm<B>> {
    type PrivateKey;
    type PublicKey;
}

pub trait SoftwareKey<B: Backend + ?Sized, A: Algorithm<B>>: AlgoParams<B, A> {
    fn new() -> Self;
}
pub trait HardwareKey<B: Backend + ?Sized, A: Algorithm<B>>: AlgoParams<B, A> {
    type Salt;
    fn new(salt: Self::Salt) -> Self;
}

pub trait KeyPairGen<A, P>: ErrorType
where
    Self: Backend,
    A: Algorithm<Self>,
    P: AlgoParams<Self, A>,
{
    fn key_pair_gen(
        &self,
        algorithm: &A,
        params: &P,
    ) -> Result<(P::PrivateKey, P::PublicKey), Self::Error>;
}

pub trait Sign<K: ?Sized>: ErrorType {
    type Message: Digest;
    type Signature;
    type Param;
    fn sign(
        &self,
        key: &K,
        message: &Self::Message,
        param: &Self::Param,
    ) -> Result<Self::Signature, Self::Error>;
}

pub trait Verify<K: ?Sized>: ErrorType {
    type Message: Digest;
    type Signature;
    type Param;
    fn verify(
        &self,
        key: &K,
        message: &Self::Message,
        param: &Self::Param,
        signature: &Self::Signature,
    ) -> Result<bool, Self::Error>;
}

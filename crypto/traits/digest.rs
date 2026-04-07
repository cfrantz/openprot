use crate::backend::Backend;
use crate::error::ErrorType;

/// The algorithm trait represents an algorithm that the backend can perform.
pub trait Algorithm<B: Backend + ?Sized> {}

/// Common algorithms for hash digests.
/// For each algorithm supported by your backend, you should implement
/// Algorithm<YourBackend> on the corresponding struct.
#[derive(Clone, Copy)]
pub struct Sha2_224;
#[derive(Clone, Copy)]
pub struct Sha2_256;
#[derive(Clone, Copy)]
pub struct Sha2_384;
#[derive(Clone, Copy)]
pub struct Sha2_512;

pub trait Digest {
    fn digest(&self) -> &[u8];
}

pub trait DigestInit<A>: ErrorType
where
    Self: Backend,
    A: Algorithm<Self>,
{
    type Context;
    fn init(&self, algorithm: &A) -> Result<Self::Context, Self::Error>;
}

pub trait DigestUpdate<Context: ?Sized>: ErrorType {
    fn update(&self, context: &Context, data: &[u8]) -> Result<(), Self::Error>;
}

pub trait DigestFinal<Context: ?Sized>: ErrorType {
    type Digest: Digest;
    fn finalize(&self, context: Context) -> Result<Self::Digest, Self::Error>;
}

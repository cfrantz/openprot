use crate::backend::Backend;
use crate::digest::Digest;
use crate::error::ErrorType;
use crate::Algorithm;

pub struct HmacSha256;
pub struct HmacSha384;
pub struct HmacSha512;

impl<B: Backend + ?Sized> Algorithm<B> for HmacSha256 {}
impl<B: Backend + ?Sized> Algorithm<B> for HmacSha384 {}
impl<B: Backend + ?Sized> Algorithm<B> for HmacSha512 {}

pub trait HmacInit<A>: ErrorType
where
    Self: Backend,
    A: Algorithm<Self>,
{
    type Key;
    type Context;
    fn hmac_init(&self, algo: &A, key: &Self::Key) -> Result<Self::Context, Self::Error>;
}

pub trait HmacUpdate<Context: ?Sized>: ErrorType {
    fn hmac_update(&self, context: &Context, data: &[u8]) -> Result<(), Self::Error>;
}

pub trait HmacFinal<Context: ?Sized>: ErrorType {
    type Tag: Digest;
    fn hmac_finalize(&self, context: Context) -> Result<Self::Tag, Self::Error>;
}

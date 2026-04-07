use crate::backend::Backend;
use crate::error::ErrorType;
use crate::Algorithm;

pub trait SymmetricKeyGen<A>: ErrorType
where
    Self: Backend,
    A: Algorithm<Self>,
{
    type Key;
    fn symmetric_key_gen(
        &self,
        algo: &A,
        personalization_string: &[u8],
    ) -> Result<Self::Key, Self::Error>;
}

pub trait HardwareKeyGen<A>: ErrorType
where
    Self: Backend,
    A: Algorithm<Self>,
{
    type Key;
    type Salt;
    fn hw_backed_key(
        &self,
        algo: &A,
        version: u32,
        salt: &Self::Salt,
    ) -> Result<Self::Key, Self::Error>;
}

pub trait KeyWrap: ErrorType {
    type Key;
    fn wrap(
        &self,
        key_to_wrap: &Self::Key,
        kek: &Self::Key,
        output: &mut [u8],
    ) -> Result<usize, Self::Error>;
    fn unwrap(&self, wrapped_key: &[u8], kek: &Self::Key) -> Result<Self::Key, Self::Error>;
}

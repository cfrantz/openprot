use crate::backend::CryptoClient;
use crate::util;
use crypto_common::Opcode;
use crypto_traits::digest::*;

use core::marker::PhantomData;
use otcrypto::HashMode;
use paste::paste;
use zerocopy::{FromZeros, IntoBytes};

//#[derive(zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes)]
//#[repr(C)]
pub struct Sha2Context<T> {
    index: u32,
    _phantom: PhantomData<T>,
}

macro_rules! hash_impl {
    ($algo:ident, $mode:expr, $op_prefix:ident, $bytesize:expr) => { paste! {

        impl Algorithm<CryptoClient> for $algo {}

        impl From<u32> for Sha2Context<$algo> {
            fn from(index: u32) -> Self {
                Self {
                    index,
                    _phantom: PhantomData,
                }
            }
        }

        impl From<Sha2Context<$algo>> for u32 {
            fn from(ctx: Sha2Context<$algo>) -> u32 {
                ctx.index
            }
        }
        #[derive(
            Clone, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes,
        )]
        #[repr(C)]
        pub struct [<$algo Digest>] {
            pub mode: HashMode,
            pub data: [u8; $bytesize],
        }

        impl [<$algo Digest>] {
            pub const fn new(data: [u8; $bytesize]) -> Self {
                [<$algo Digest>] {
                    mode: $mode,
                    data,
                }
            }
        }

        impl Digest for [<$algo Digest>] {
            fn digest(&self) -> &[u8] {
                &self.data
            }
        }

        impl DigestInit<$algo> for CryptoClient {
            type Context = Sha2Context<$algo>;
            fn init(&self, _algorithm: &$algo) -> Result<Self::Context, Self::Error> {
                let index = util::digest::init(self, Opcode::[<$op_prefix _INIT>])?;
                Ok(Sha2Context {
                    index,
                    _phantom: PhantomData,
                })
            }
        }

        impl DigestUpdate<Sha2Context<$algo>> for CryptoClient {
            fn update(&self, context: &Sha2Context<$algo>, data: &[u8]) -> Result<(), Self::Error> {
                util::digest::update(self, Opcode::[<$op_prefix _UPDATE>], context.index, data)
            }
        }

        impl DigestFinal<Sha2Context<$algo>> for CryptoClient {
            type Digest = [<$algo Digest>];
            fn finalize(&self, context: Sha2Context<$algo>) -> Result<Self::Digest, Self::Error> {
                let mut digest = [<$algo Digest>]::new_zeroed();
                util::digest::finalize(
                    self,
                    Opcode::[<$op_prefix _FINAL>],
                    context.index,
                    digest.as_mut_bytes(),
                )?;
                Ok(digest)
            }
        }
    }}
}

hash_impl!(Sha2_256, HashMode::Sha256, SHA2_256, 32);
hash_impl!(Sha2_384, HashMode::Sha384, SHA2_384, 48);
hash_impl!(Sha2_512, HashMode::Sha512, SHA2_512, 64);

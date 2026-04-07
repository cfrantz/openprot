use crate::backend::CryptoClient;
use crate::util;
use crypto_common::Opcode;
use crypto_traits::digest::Digest;
use crypto_traits::hmac::*;

use core::marker::PhantomData;
use otcrypto::{HashMode, KeyMode};
use paste::paste;
use pw_status::Result;
use zerocopy::{FromBytes, FromZeros, IntoBytes};

pub struct HmacContext<T> {
    index: u32,
    _phantom: PhantomData<T>,
}

fn hash_mode_to_key_mode(mode: HashMode) -> KeyMode {
    match mode {
        HashMode::Sha256 => KeyMode::HmacSha256,
        HashMode::Sha384 => KeyMode::HmacSha384,
        HashMode::Sha512 => KeyMode::HmacSha512,
        _ => unreachable!(),
    }
}

macro_rules! hmac_impl {
    ($algo:ident, $mode:expr, $op_prefix:ident, $bytesize:expr) => { paste! {

        pub struct [<$algo Key>](pub [u8; $bytesize]);

        #[derive(
            Clone, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes,
        )]
        #[repr(C)]
        pub struct [<$algo Tag>] {
            pub mode: HashMode,
            pub data: [u8; $bytesize],
        }

        impl [<$algo Tag>] {
            pub const fn new(data: [u8; $bytesize]) -> Self {
                [<$algo Tag>] {
                    mode: $mode,
                    data,
                }
            }
        }

        impl Digest for [<$algo Tag>] {
            fn digest(&self) -> &[u8] {
                &self.data
            }
        }

        impl<'a> HmacInit<'a, $algo> for CryptoClient {
            type Key = [<$algo Key>];
            type Context = HmacContext<$algo>;
            fn hmac_init(&self, _algorithm: &$algo, key_wrapper: &Self::Key) -> Result<Self::Context> {
                use otcrypto::{BlindedKey, KeyConfig, LibVersion, HardenedBool, KeySecurityLevel};
                let key_material = &key_wrapper.0;
                let blinded_key = BlindedKey {
                    config: KeyConfig {
                        version: LibVersion::_1,
                        key_mode: hash_mode_to_key_mode($mode),
                        key_length: key_material.len() as u32,
                        hw_backed: HardenedBool::False,
                        exportable: HardenedBool::True,
                        security_level: KeySecurityLevel::Low,
                    },
                    keyblob_length: (key_material.len() * 2) as u32,
                    keyblob: 0,
                    checksum: 0,
                };

                let mut buf = [0u8; 128 + core::mem::size_of::<BlindedKey>()];
                let (bk, rest) = BlindedKey::mut_from_prefix(&mut buf).map_err(|_| pw_status::Error::Internal)?;
                *bk = blinded_key;
                bk.with_internal_key_material();
                rest[..key_material.len()].copy_from_slice(key_material);
                for x in &mut rest[key_material.len()..key_material.len()*2] { *x = 0; }

                let index = util::hmac::init(self, Opcode::[<HMAC_ $op_prefix _INIT>], &buf[..core::mem::size_of::<BlindedKey>() + key_material.len()*2])?;
                Ok(HmacContext {
                    index,
                    _phantom: PhantomData,
                })
            }
        }

        impl HmacUpdate<HmacContext<$algo>> for CryptoClient {
            fn hmac_update(&self, context: &HmacContext<$algo>, data: &[u8]) -> Result<()> {
                util::hmac::update(self, Opcode::[<HMAC_ $op_prefix _UPDATE>], context.index, data)
            }
        }

        impl HmacFinal<HmacContext<$algo>> for CryptoClient {
            type Tag = [<$algo Tag>];
            fn hmac_finalize(&self, context: HmacContext<$algo>) -> Result<Self::Tag> {
                let mut tag = [<$algo Tag>]::new_zeroed();
                util::hmac::finalize(
                    self,
                    Opcode::[<HMAC_ $op_prefix _FINAL>],
                    context.index,
                    tag.as_mut_bytes(),
                )?;
                Ok(tag)
            }
        }
    }}
}

hmac_impl!(HmacSha256, HashMode::Sha256, SHA2_256, 32);
hmac_impl!(HmacSha384, HashMode::Sha384, SHA2_384, 48);
hmac_impl!(HmacSha512, HashMode::Sha512, SHA2_512, 64);

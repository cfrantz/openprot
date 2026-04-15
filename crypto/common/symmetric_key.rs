use crypto_traits::error::ErrorType;

use otcrypto::{BlindedKey, HardenedBool, KeyConfig, KeySecurityLevel, LibVersion};
// Re-export for convenience
pub use otcrypto::KeyMode;
use paste::paste;
use pw_status::{Error, Result};
use zerocopy::FromBytes;

macro_rules! symmetric_key {
    ($algo:ident, $keylen_bits:expr, $blindlen:expr) => {
        paste! {
            #[derive(
                Debug,
                Clone,
                zerocopy::KnownLayout,
                zerocopy::Immutable,
                zerocopy::FromBytes,
                zerocopy::IntoBytes,
            )]
            #[repr(C)]
            pub struct [<$algo Key>] {
                pub key: BlindedKey,
                pub data: [u8; $blindlen],
            }
            impl ErrorType for [<$algo Key>] {
                type Error = Error;
            }
            impl [<$algo Key>] {
                pub const KEY_SIZE_BITS: usize = $keylen_bits;

                pub fn new(key_mode: KeyMode, hw_backed: HardenedBool, exportable: HardenedBool, security_level: KeySecurityLevel) -> Self {
                    Self {
                        key: BlindedKey {
                            config: KeyConfig {
                                version: LibVersion::_1,
                                key_mode,
                                key_length: $keylen_bits / 8,
                                hw_backed,
                                exportable,
                                security_level,
                            },
                            keyblob_length: $blindlen,
                            keyblob: 0,
                            checksum: 0,
                        },
                        data: [0u8; $blindlen],
                    }
                }

                pub fn integrity_checksum(&self) -> u32 {
                    let mut checksum = util_misc::Crc32::new();

                    checksum.add32(u32::from(self.key.config.version));
                    checksum.add32(u32::from(self.key.config.key_mode));
                    checksum.add32(u32::from(self.key.config.key_length));
                    checksum.add32(u32::from(self.key.config.hw_backed));
                    checksum.add32(u32::from(self.key.config.exportable));
                    checksum.add32(u32::from(self.key.config.security_level));
                    checksum.add32(u32::from(self.key.keyblob_length));
                    for k in self.data[..(self.key.keyblob_length as usize)].chunks(4) {
                        checksum.add32(u32::from_le_bytes(k.try_into().unwrap()));
                    }
                    checksum.finalize()
                }

                pub fn with_key_material(key_mode: KeyMode, key_material: &[u8]) -> Self {
                    let mut key = Self {
                        key: BlindedKey {
                            config: KeyConfig {
                                version: LibVersion::_1,
                                key_mode,
                                key_length: $keylen_bits / 8,
                                hw_backed: HardenedBool::False,
                                exportable: HardenedBool::True,
                                security_level: KeySecurityLevel::Low,
                            },
                            keyblob_length: $blindlen,
                            keyblob: 0,
                            checksum: 0,
                        },
                        data: [0u8; $blindlen],
                    };
                    key.data[..$blindlen/2].copy_from_slice(key_material);
                    key.key.checksum = key.integrity_checksum();
                    key
                }

                pub fn new_mutref(buf: &mut [u8], key_mode: KeyMode, hw_backed: HardenedBool, exportable: HardenedBool, security_level: KeySecurityLevel) -> Result<(&mut Self, &mut [u8])> {
                    let (this, rest) = Self::mut_from_prefix(buf).map_err(|_| Error::Internal)?;
                    this.key = BlindedKey {
                            config: KeyConfig {
                                version: LibVersion::_1,
                                key_mode,
                                key_length: $keylen_bits/ 8,
                                hw_backed,
                                exportable,
                                security_level,
                            },
                            keyblob_length: $blindlen,
                            keyblob: 0,
                            checksum: 0,
                    };
                    for x in this.data.iter_mut() { *x=0; }
                    Ok((this, rest))
                }
            }

        }
    };
}

// basename, keylen_bits, blindlen
symmetric_key!(Aes128, 128, 32);
symmetric_key!(Aes192, 192, 48);
symmetric_key!(Aes256, 256, 64);

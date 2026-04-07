use crypto_traits::error::ErrorType;

use otcrypto::{
    BlindedKey, DiceDiversifier, DiceKeymgrDiversifier, HardenedBool, KeyConfig, KeyMode,
    KeySecurityLevel, LibVersion, UnblindedKey,
};
use paste::paste;
use pw_status::{Error, Result};
use zerocopy::FromBytes;

macro_rules! keypair {
    ($algo:ident, $publen:expr, $privlen:expr, $blindlen:expr) => {
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
            pub struct [<$algo PublicKey>] {
                pub key: UnblindedKey,
                pub data: [u8; $publen],
            }
            impl ErrorType for [<$algo PublicKey>] {
                type Error = Error;
            }
            impl [<$algo PublicKey>] {
                pub const PUBLIC_KEY_SIZE: usize = $publen;
                pub const PRIVATE_KEY_SIZE: usize = $privlen;
                pub const BLINDED_KEY_SIZE: usize = $blindlen;

                pub fn new(key_mode: KeyMode) -> Self {
                    Self {
                        key: UnblindedKey {
                            key_mode,
                            key_length: $publen,
                            key: 0,
                            checksum: 0,
                        },
                        data: [0u8; $publen],
                    }
                }
                pub fn new_mutref(buf: &mut [u8], key_mode: KeyMode) -> Result<(&mut Self, &mut [u8])> {
                    let (this, rest) = Self::mut_from_prefix(buf).map_err(|_| Error::Internal)?;
                    this.key = UnblindedKey {
                            key_mode,
                            key_length: $publen,
                            key: 0,
                            checksum: 0,
                    };
                    for x in this.data.iter_mut() { *x=0; }
                    Ok((this, rest))
                }
            }


            #[derive(
                Debug,
                Clone,
                zerocopy::KnownLayout,
                zerocopy::Immutable,
                zerocopy::FromBytes,
                zerocopy::IntoBytes,
            )]
            #[repr(C)]
            pub struct [<$algo PrivateKey>] {
                pub key: BlindedKey,
                pub data: [u8; $blindlen],
            }
            impl ErrorType for [<$algo PrivateKey>] {
                type Error = Error;
            }
            impl [<$algo PrivateKey>] {
                pub const PUBLIC_KEY_SIZE: usize = $publen;
                pub const PRIVATE_KEY_SIZE: usize = $privlen;
                pub const BLINDED_KEY_SIZE: usize = $blindlen;

                pub fn new(key_mode: KeyMode, hw_backed: HardenedBool, exportable: HardenedBool, security_level: KeySecurityLevel) -> Self {
                    Self {
                        key: BlindedKey {
                            config: KeyConfig {
                                version: LibVersion::_1,
                                key_mode,
                                key_length: $privlen,
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
                pub fn new_mutref(buf: &mut [u8], key_mode: KeyMode, hw_backed: HardenedBool, exportable: HardenedBool, security_level: KeySecurityLevel) -> Result<(&mut Self, &mut [u8])> {
                    let (this, rest) = Self::mut_from_prefix(buf).map_err(|_| Error::Internal)?;
                    this.key = BlindedKey {
                            config: KeyConfig {
                                version: LibVersion::_1,
                                key_mode,
                                key_length: $privlen,
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

macro_rules! signing_keypair {
    ($algo:ident, $publen:expr, $privlen:expr, $blindlen:expr) => {
        keypair!($algo, $publen, $privlen, $blindlen);
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
            pub struct [<$algo Signature>] {
                pub data: [u8; $publen],
            }
            impl Default for [<$algo Signature>] {
                fn default() -> Self {
                    Self {
                        data: [0u8; $publen],
                    }
                }
            }
        }
    };
}

// basename, pubkey_size, privkey_size, blinded_sz
signing_keypair!(EcdsaP256, 32 * 2, 32, 40 * 2);
signing_keypair!(EcdsaP384, 48 * 2, 48, 56 * 2);
signing_keypair!(Ed25519, 32 * 2, 32, 40 * 2);
signing_keypair!(Rsa2048, 256, 256, 768);
signing_keypair!(Rsa3072, 384, 384, 1152);
signing_keypair!(Rsa4096, 512, 512, 1536);
keypair!(EcdhP256, 32 * 2, 32, 40 * 2);
keypair!(EcdhP384, 48 * 2, 48, 56 * 2);
keypair!(X25519, 32 * 2, 32, 40 * 2);

#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct DiceP256PrivateKey {
    pub key: BlindedKey,
    pub data: DiceDiversifier,
}
impl ErrorType for DiceP256PrivateKey {
    type Error = Error;
}
impl DiceP256PrivateKey {
    pub const PUBLIC_KEY_SIZE: usize = 64;
    pub const PRIVATE_KEY_SIZE: usize = 32;
    pub const BLINDED_KEY_SIZE: usize = core::mem::size_of::<DiceDiversifier>();

    pub fn new(diversifier: DiceKeymgrDiversifier, attestation_seed: [u32; 16]) -> Self {
        Self {
            key: BlindedKey {
                config: KeyConfig {
                    version: LibVersion::_1,
                    key_mode: KeyMode::EcdsaP256,
                    key_length: 32,
                    hw_backed: HardenedBool::True,
                    exportable: HardenedBool::False,
                    security_level: KeySecurityLevel::Low,
                },
                keyblob_length: Self::BLINDED_KEY_SIZE as u32,
                keyblob: 0,
                checksum: 0,
            },
            data: DiceDiversifier {
                diversifier,
                attestation_seed,
            },
        }
    }
    pub fn new_mutref(
        buf: &mut [u8],
        diversifier: DiceKeymgrDiversifier,
        attestation_seed: [u32; 16],
    ) -> Result<(&mut Self, &mut [u8])> {
        let (this, rest) = Self::mut_from_prefix(buf).map_err(|_| Error::Internal)?;
        this.key = BlindedKey {
            config: KeyConfig {
                version: LibVersion::_1,
                key_mode: KeyMode::EcdsaP256,
                key_length: 32,
                hw_backed: HardenedBool::True,
                exportable: HardenedBool::False,
                security_level: KeySecurityLevel::Low,
            },
            keyblob_length: Self::BLINDED_KEY_SIZE as u32,
            keyblob: 0,
            checksum: 0,
        };
        this.data = DiceDiversifier {
            diversifier,
            attestation_seed,
        };
        Ok((this, rest))
    }
}

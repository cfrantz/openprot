use crate::backend::CryptoClient;
use crate::sha2::Sha2_256Digest;
use crate::util;
use crypto_common::keytypes::{
    DiceP256PrivateKey, EcdsaP256PrivateKey, EcdsaP256PublicKey, EcdsaP256Signature,
};
use crypto_common::Opcode;
use crypto_traits::asymmetric;
use crypto_traits::NoParam;

use otcrypto::{
    DiceKeymgrDiversifier, HardenedBool, KeyConfig, KeyMode, KeySecurityLevel, LibVersion,
};
use zerocopy::{FromBytes, FromZeros, Immutable, IntoBytes, KnownLayout};

impl asymmetric::Algorithm<CryptoClient> for asymmetric::EcdsaP256 {}

pub struct SoftwareKey;

#[derive(Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct HardwareKey {
    pub version: u32,
    pub salt: [u32; 7],
}

impl asymmetric::AlgoParams<CryptoClient, asymmetric::EcdsaP256> for SoftwareKey {
    type PrivateKey = EcdsaP256PrivateKey;
    type PublicKey = EcdsaP256PublicKey;
}
impl asymmetric::SoftwareKey<CryptoClient, asymmetric::EcdsaP256> for SoftwareKey {
    fn new() -> SoftwareKey {
        SoftwareKey
    }
}

impl asymmetric::AlgoParams<CryptoClient, asymmetric::EcdsaP256> for HardwareKey {
    type PrivateKey = EcdsaP256PrivateKey;
    type PublicKey = EcdsaP256PublicKey;
}
impl asymmetric::HardwareKey<CryptoClient, asymmetric::EcdsaP256> for HardwareKey {
    type Salt = [u32; 7];
    fn new(salt: Self::Salt) -> HardwareKey {
        HardwareKey { version: 0, salt }
    }
}

impl asymmetric::KeyPairGen<asymmetric::EcdsaP256, SoftwareKey> for CryptoClient {
    fn key_pair_gen(
        &self,
        _algorithm: &asymmetric::EcdsaP256,
        _params: &SoftwareKey,
    ) -> Result<(EcdsaP256PrivateKey, EcdsaP256PublicKey), Self::Error> {
        let mut private = EcdsaP256PrivateKey::new_zeroed();
        let mut public = EcdsaP256PublicKey::new_zeroed();
        util::asymmetric::keygen(
            self,
            Opcode::ECDSA_P256_KEYGEN,
            &KeyConfig {
                version: LibVersion::_1,
                key_mode: KeyMode::EcdsaP256,
                key_length: EcdsaP256PrivateKey::PRIVATE_KEY_SIZE as u32,
                hw_backed: HardenedBool::False,
                exportable: HardenedBool::True,
                security_level: KeySecurityLevel::Low,
            },
            /*salt=*/ None,
            private.as_mut_bytes(),
            public.as_mut_bytes(),
        )?;
        Ok((private, public))
    }
}

impl asymmetric::KeyPairGen<asymmetric::EcdsaP256, HardwareKey> for CryptoClient {
    fn key_pair_gen(
        &self,
        _algorithm: &asymmetric::EcdsaP256,
        params: &HardwareKey,
    ) -> Result<(EcdsaP256PrivateKey, EcdsaP256PublicKey), Self::Error> {
        let mut private = EcdsaP256PrivateKey::new_zeroed();
        let mut public = EcdsaP256PublicKey::new_zeroed();
        util::asymmetric::keygen(
            self,
            Opcode::ECDSA_P256_KEYGEN,
            &KeyConfig {
                version: LibVersion::_1,
                key_mode: KeyMode::EcdsaP256,
                key_length: EcdsaP256PrivateKey::PRIVATE_KEY_SIZE as u32,
                hw_backed: HardenedBool::True,
                exportable: HardenedBool::False,
                security_level: KeySecurityLevel::Low,
            },
            /*salt=*/ Some(params.as_bytes()),
            private.as_mut_bytes(),
            public.as_mut_bytes(),
        )?;
        Ok((private, public))
    }
}

impl asymmetric::Sign<EcdsaP256PrivateKey> for CryptoClient {
    type Message = Sha2_256Digest;
    type Signature = EcdsaP256Signature;
    type Param = NoParam;
    fn sign(
        &self,
        key: &EcdsaP256PrivateKey,
        message: &Self::Message,
        _param: &Self::Param,
    ) -> Result<Self::Signature, Self::Error> {
        let mut signature = EcdsaP256Signature::new_zeroed();
        util::asymmetric::sign(
            self,
            Opcode::ECDSA_P256_SIGN,
            key.as_bytes(),
            &[],
            message.as_bytes(),
            signature.as_mut_bytes(),
        )?;
        Ok(signature)
    }
}

impl asymmetric::Verify<EcdsaP256PublicKey> for CryptoClient {
    type Message = Sha2_256Digest;
    type Signature = EcdsaP256Signature;
    type Param = NoParam;
    fn verify(
        &self,
        key: &EcdsaP256PublicKey,
        message: &Self::Message,
        _param: &Self::Param,
        signature: &Self::Signature,
    ) -> Result<bool, Self::Error> {
        util::asymmetric::verify(
            self,
            Opcode::ECDSA_P256_VERIFY,
            key.as_bytes(),
            &[],
            message.as_bytes(),
            signature.as_bytes(),
        )
    }
}

impl asymmetric::ShareSecret<EcdsaP256PrivateKey, EcdsaP256PublicKey> for CryptoClient {
    type Secret = EcdsaP256PrivateKey;
    fn share_secret(
        &self,
        secret_key: &EcdsaP256PrivateKey,
        public_key: &EcdsaP256PublicKey,
    ) -> Result<Self::Secret, Self::Error> {
        let mut secret = EcdsaP256PrivateKey::new_zeroed();
        util::asymmetric::share_secret(
            self,
            Opcode::ECDH_P256_KEY_AGREEMENT,
            secret_key.as_bytes(),
            public_key.as_bytes(),
            secret.as_mut_bytes(),
        )?;
        Ok(secret)
    }
}

#[derive(Clone)]
pub struct DiceKey {
    pub diversifier: DiceKeymgrDiversifier,
    pub seed: [u32; 16],
}

impl DiceKey {
    pub fn new(salt: [u32; 8], version: u32, seed: &[u32]) -> Self {
        let mut dk = DiceKey {
            diversifier: DiceKeymgrDiversifier { salt, version },
            seed: Default::default(),
        };
        dk.seed[..10].copy_from_slice(seed);
        dk
    }
}

impl asymmetric::AlgoParams<CryptoClient, asymmetric::EcdsaP256> for DiceKey {
    type PrivateKey = DiceP256PrivateKey;
    type PublicKey = EcdsaP256PublicKey;
}
//impl asymmetric::HardwareKey<CryptoClient, asymmetric::EcdsaP256> for DiceKey {
//    type Salt = [u8; 32];
//    fn new(salt: Self::Salt) -> HardwareKey {
//        HardwareKey { salt }
//    }
//}

impl asymmetric::KeyPairGen<asymmetric::EcdsaP256, DiceKey> for CryptoClient {
    fn key_pair_gen(
        &self,
        _algorithm: &asymmetric::EcdsaP256,
        params: &DiceKey,
    ) -> Result<(DiceP256PrivateKey, EcdsaP256PublicKey), Self::Error> {
        let mut private = DiceP256PrivateKey::new(params.diversifier.clone(), params.seed.clone());
        let mut public = EcdsaP256PublicKey::new_zeroed();
        util::asymmetric::keygen(
            self,
            Opcode::DICE_P256_KEYGEN,
            &private.key.config.clone(),
            None,
            private.as_mut_bytes(),
            public.as_mut_bytes(),
        )?;
        Ok((private, public))
    }
}

impl asymmetric::Sign<DiceP256PrivateKey> for CryptoClient {
    type Message = Sha2_256Digest;
    type Signature = EcdsaP256Signature;
    type Param = NoParam;
    fn sign(
        &self,
        key: &DiceP256PrivateKey,
        message: &Self::Message,
        _param: &Self::Param,
    ) -> Result<Self::Signature, Self::Error> {
        let mut signature = EcdsaP256Signature::new_zeroed();
        util::asymmetric::sign(
            self,
            Opcode::DICE_P256_SIGN,
            key.as_bytes(),
            &[],
            message.as_bytes(),
            signature.as_mut_bytes(),
        )?;
        Ok(signature)
    }
}

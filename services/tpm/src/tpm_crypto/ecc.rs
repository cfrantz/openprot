#![allow(unused_imports)]
use crate::tpm_crypto::NullCrypto;
use crypto::{ecc::TpmEcc, implement_tpm_ecc, rand::TpmRand};
use crypto_client::backend::CryptoClient;
use crypto_client::ecdsa::HardwareKey;
use crypto_client::sha2::Sha2_256Digest;
use crypto_common::keytypes::{EcdsaP256PrivateKey, EcdsaP256PublicKey, EcdsaP256Signature};
use crypto_common::symmetric_key::KeyMode;
use crypto_traits::asymmetric::{
    AlgoParams, Algorithm as EcdsaAlgorithm, EcdsaP256, HardwareKey as _, KeyPairGen, ShareSecret,
    Sign, Verify,
};
use crypto_traits::NoParam;
use tpm_types::*;

//use zerocopy::{FromBytes, IntoBytes};

const ECC_CURVE_PARAMS: [TpmEccCurveMetadata; 1] = [TpmEccCurveMetadata {
    curve_id: TpmEccCurve::NistP256,
    key_size_bits: 256,
    kdf: TpmtKdfScheme {
        scheme: TpmAlgId::Kdf1_56A,
        details: TpmAlgId::Sha2_256,
    },
    sign: TpmtEccScheme {
        scheme: TpmAlgId::Null,
        details: TpmAlgId::Null,
    },
    oid: &[],
    p: Tpm2BEccParameter::new(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ]),
    a: Tpm2BEccParameter::new(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFC,
    ]),
    b: Tpm2BEccParameter::new(&[
        0x5A, 0xC6, 0x35, 0xD8, 0xAA, 0x3A, 0x93, 0xE7, 0xB3, 0xEB, 0xBD, 0x55, 0x76, 0x98, 0x86,
        0xBC, 0x65, 0x1D, 0x06, 0xB0, 0xCC, 0x53, 0xB0, 0xF6, 0x3B, 0xCE, 0x3C, 0x3E, 0x27, 0xD2,
        0x60, 0x4B,
    ]),
    gx: Tpm2BEccParameter::new(&[
        0x6B, 0x17, 0xD1, 0xF2, 0xE1, 0x2C, 0x42, 0x47, 0xF8, 0xBC, 0xE6, 0xE5, 0x63, 0xA4, 0x40,
        0xF2, 0x77, 0x03, 0x7D, 0x81, 0x2D, 0xEB, 0x33, 0xA0, 0xF4, 0xA1, 0x39, 0x45, 0xD8, 0x98,
        0xC2, 0x96,
    ]),
    gy: Tpm2BEccParameter::new(&[
        0x4F, 0xE3, 0x42, 0xE2, 0xFE, 0x1A, 0x7F, 0x9B, 0x8E, 0xE7, 0xEB, 0x4A, 0x7C, 0x0F, 0x9E,
        0x16, 0x2B, 0xCE, 0x33, 0x57, 0x6B, 0x31, 0x5E, 0xCE, 0xCB, 0xB6, 0x40, 0x68, 0x37, 0xBF,
        0x51, 0xF5,
    ]),
    n: Tpm2BEccParameter::new(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xBC, 0xE6, 0xFA, 0xAD, 0xA7, 0x17, 0x9E, 0x84, 0xF3, 0xB9, 0xCA, 0xC2, 0xFC, 0x63,
        0x25, 0x51,
    ]),
    h: Tpm2BEccParameter::new(&[1]),
}];

impl TpmEcc for NullCrypto {
    fn ecc_subsystem_init(&self) -> bool {
        true
    }
    fn ecc_subsystem_startup(&self) -> bool {
        true
    }
    fn ecc_parameters_by_index(&self, index: usize) -> Option<&'static TpmEccCurveMetadata> {
        ECC_CURVE_PARAMS.get(index)
    }
    fn ecc_is_valid_private_key(&self, _d: &[u8], _curve_id: TpmEccCurve) -> bool {
        true
    }
    fn ecc_is_point_on_curve(&self, _curve_id: TpmEccCurve, _q: &TpmsEccPoint) -> bool {
        true
    }
    fn ecc_new_key_pair(
        &self,
        qout: &mut TpmsEccPoint,
        dout: &mut Tpm2BEccParameter,
        _curve_id: TpmEccCurve,
    ) -> TpmRc {
        // TODO: ensure curve is P256.
        pw_log::info!("ecc_new_key_pair");
        let mut key = [0u32; 7];
        let _ = <Self as TpmRand>::rand_drbg_generate(self, None, key.as_mut_bytes());
        let (private, mut public) =
            match self.client.key_pair_gen(&EcdsaP256, &HardwareKey::new(key)) {
                Ok((private, public)) => (private, public),
                Err(e) => {
                    pw_log::error!("ecc new_key_pair: {}", e as u32);
                    return TpmRc::Failure;
                }
            };

        // Little-endian to crypto-endian
        public.data[..32].reverse();
        public.data[32..64].reverse();
        dout.copy_from_slice(&private.data[..32]);
        qout.x.copy_from_slice(&public.data[..32]);
        qout.y.copy_from_slice(&public.data[32..64]);
        TpmRc::Success
    }
    fn ecc_generate_key(
        &self,
        public_area: &mut TpmtPublic,
        sensitive: &mut TpmtSensitive,
        rand: Option<&mut RandState>,
    ) -> TpmRc {
        pw_log::info!("ecc_generate_key");
        // TODO: check that curve_id is NIST_P256
        let (pk, _) = TpmsEccPoint::mut_from_prefix(&mut public_area.unique).expect("public_area");
        let (sk, _) =
            Tpm2BEccParameter::mut_from_prefix(&mut sensitive.sensitive).expect("sensitive");
        let (_param, _) =
            TpmsEccParms::ref_from_prefix(&public_area.parameters).expect("parameters");
        let mut key = [0u32; 7];
        let _ = <Self as TpmRand>::rand_drbg_generate(self, rand, key.as_mut_bytes());
        let (private, mut public) =
            match self.client.key_pair_gen(&EcdsaP256, &HardwareKey::new(key)) {
                Ok((private, public)) => (private, public),
                Err(e) => {
                    pw_log::error!("ecc generate_key: {}", e as u32);
                    return TpmRc::Failure;
                }
            };
        // Little-endian to crypto-endian
        public.data[..32].reverse();
        public.data[32..64].reverse();
        sk.copy_from_slice(&private.data[..32]);
        pk.x.copy_from_slice(&public.data[..32]);
        pk.y.copy_from_slice(&public.data[32..64]);
        TpmRc::Success
    }
    fn ecc_sign(
        &self,
        signature: &mut TpmtSignature,
        sign_key: &TpmObject,
        digest: &[u8],
        _scheme: &TpmtEccScheme,
        _rand: Option<&mut RandState>,
    ) -> TpmRc {
        // TODO: check that sign_alg is ECDSA
        // TODO: check that curve_id is NIST_P256
        pw_log::info!("ecc_sign");
        let (sig, _) =
            TpmsSignatureEcc::mut_from_prefix(&mut signature.signature).expect("signature");
        let (sk, _) =
            Tpm2BEccParameter::ref_from_prefix(&sign_key.sensitive.sensitive).expect("sensitive");

        let (key, _) = <[u32; 8]>::ref_from_prefix(sk.as_slice()).unwrap();
        let key = EcdsaP256PrivateKey::new_hw(KeyMode::EcdsaP256, key[0], &key[1..]);
        let digest = Sha2_256Digest::new(digest.try_into().unwrap());
        match self.client.sign(&key, &digest, &NoParam) {
            Ok(mut signature) => {
                let half = signature.data.len() / 2;
                // little-endian to crypto endian
                signature.data[..half].reverse();
                signature.data[half..].reverse();
                sig.r.copy_from_slice(&signature.data[..half]);
                sig.s.copy_from_slice(&signature.data[half..]);
                TpmRc::Success
            }
            Err(e) => {
                pw_log::error!("ecc sign: {}", e as u32);
                return TpmRc::Failure;
            }
        }
    }
    fn ecc_verify(&self, signature: &TpmtSignature, sign_key: &TpmObject, digest: &[u8]) -> TpmRc {
        // TODO: check that sign_alg is ECDSA
        // TODO: check that curve_id is NIST_P256
        pw_log::info!("ecc_verify");
        let (sig, _) = TpmsSignatureEcc::ref_from_prefix(&signature.signature).expect("signature");
        let (pk, _) =
            TpmsEccPoint::ref_from_prefix(&sign_key.public_area.unique).expect("public_area");
        let digest = Sha2_256Digest::new(digest.try_into().unwrap());
        let mut signature = EcdsaP256Signature::default();
        signature.data[0..32].copy_from_slice(sig.r.as_slice());
        signature.data[32..64].copy_from_slice(sig.s.as_slice());
        signature.data[0..32].reverse();
        signature.data[32..64].reverse();
        let mut key = EcdsaP256PublicKey::new(KeyMode::EcdsaP256);
        key.data[0..32].copy_from_slice(pk.x.as_slice());
        key.data[32..64].copy_from_slice(pk.y.as_slice());
        key.data[0..32].reverse();
        key.data[32..64].reverse();
        key.key.checksum = key.calculate_checksum();

        match self.client.verify(&key, &digest, &NoParam, &signature) {
            Ok(true) => TpmRc::Success,
            Ok(false) => {
                pw_log::info!("signature verify returns false");
                TpmRc::Failure
            }
            Err(e) => {
                pw_log::error!("ecc verify: {}", e as u32);
                TpmRc::Failure
            }
        }
    }
    fn ecc_point_multiply(
        &self,
        rout: &mut TpmsEccPoint,
        _curve_id: TpmEccCurve,
        p: Option<&TpmsEccPoint>,
        d: &[u8],
        _q: Option<&TpmsEccPoint>,
        _u: &[u8],
    ) -> TpmRc {
        let (sk, _) = <[u32; 8]>::ref_from_prefix(d).unwrap();
        let sk = EcdsaP256PrivateKey::new_hw(KeyMode::EcdsaP256, sk[0], &sk[1..]);

        let Some(p) = p else {
            pw_log::error!("Point multiply only supported with p & d");
            return TpmRc::Failure;
        };

        let mut pk = EcdsaP256PublicKey::new(KeyMode::EcdsaP256);
        pk.data[0..32].copy_from_slice(p.x.as_slice());
        pk.data[32..64].copy_from_slice(p.y.as_slice());
        pk.data[0..32].reverse();
        pk.data[32..64].reverse();
        pk.key.checksum = pk.calculate_checksum();

        let secret = match self.client.share_secret(&sk, &pk) {
            Ok(s) => s,
            Err(e) => {
                pw_log::error!("ecc point mult: {}", e as u32);
                return TpmRc::Failure;
            }
        };

        pw_log::info!("blinded secret:");
        util_misc::hexdump(&secret.data);
        let len = secret.data.len() / 2;
        let mut data = [0u8; 40];
        for i in 0..len {
            let a = secret.data[i];
            let b = secret.data[len + i];
            data[i] = a ^ b;
        }
        pw_log::info!("unblinded secret:");
        util_misc::hexdump(&data);
        rout.x.copy_from_slice(&data[..32]);
        TpmRc::Success
    }
    fn ecc_commit_compute(
        &self,
        _k_out: &mut TpmsEccPoint,
        _l_out: &mut TpmsEccPoint,
        _e_out: &mut TpmsEccPoint,
        _curve_id: TpmEccCurve,
        _m_in: Option<&TpmsEccPoint>,
        _b_in: Option<&TpmsEccPoint>,
        _d_in: Option<&Tpm2BEccParameter>,
        _r_in: &Tpm2BEccParameter,
    ) -> TpmRc {
        TpmRc::Failure
    }
    fn ecc_encrypt(
        &self,
        _key: &TpmObject,
        _scheme: &TpmtKdfScheme,
        _plaintext: &[u8],
        _c1: &mut TpmsEccPoint,
        _c2: &mut Tpm2B,
        _c3: &mut Tpm2B,
    ) -> TpmRc {
        TpmRc::Failure
    }
    fn ecc_decrypt(
        &self,
        _key: &TpmObject,
        _scheme: &TpmtKdfScheme,
        _plaintext: &mut Tpm2B,
        _c1: &TpmsEccPoint,
        _c2: &[u8],
        _c3: &[u8],
    ) -> TpmRc {
        TpmRc::Failure
    }
    fn ecc_two_phase_key_exchange(
        &self,
        _z1_out: &mut TpmsEccPoint,
        _z2_out: &mut TpmsEccPoint,
        _curve_id: TpmEccCurve,
        _scheme: TpmAlgId,
        _ds_a: &Tpm2BEccParameter,
        _de_a: &Tpm2BEccParameter,
        _qs_b: &TpmsEccPoint,
        _qe_b: &TpmsEccPoint,
    ) -> TpmRc {
        TpmRc::Failure
    }
    fn sm2_key_exchange(
        &self,
        _z_out: &mut TpmsEccPoint,
        _curve_id: TpmEccCurve,
        _ds_a: &Tpm2BEccParameter,
        _de_a: &Tpm2BEccParameter,
        _qs_b: &TpmsEccPoint,
        _qe_b: &TpmsEccPoint,
    ) -> TpmRc {
        TpmRc::Failure
    }
}

implement_tpm_ecc!(NullCrypto);

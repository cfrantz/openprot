use crate::tpm_crypto::NullCrypto;
use crypto::{hash::TpmHash, implement_tpm_hash};
use tpm_types::*;

use crypto_client::hmac::HmacContext;
use crypto_client::sha2::Sha2Context;
use crypto_traits::digest::{
    Digest, DigestFinal, DigestInit, DigestUpdate, Sha2_256, Sha2_384, Sha2_512,
};
use crypto_traits::hmac::{HmacFinal, HmacInit, HmacSha256, HmacSha384, HmacSha512, HmacUpdate};

const HASH_ALGS: usize = 3;
const HASH_DEFINITIONS: [HashDef; HASH_ALGS + 1] = [
    HashDef {
        // SHA2-256
        method: HashMethod::empty(),
        block_size: 64u16,
        digest_size: 32u16,
        context_size: size_of::<AnyHashState>() as u16,
        hash_alg: TpmAlgId::Sha2_256,
        oid: unsafe { OID_SHA256.as_ptr() },
        pkcs1: unsafe { OID_PKCS1_SHA256.as_ptr() },
        ecdsa: unsafe { OID_ECDSA_SHA256.as_ptr() },
    },
    HashDef {
        // SHA2-384
        method: HashMethod::empty(),
        block_size: 128u16,
        digest_size: 48u16,
        context_size: size_of::<AnyHashState>() as u16,
        hash_alg: TpmAlgId::Sha2_384,
        oid: unsafe { OID_SHA384.as_ptr() },
        pkcs1: unsafe { OID_PKCS1_SHA384.as_ptr() },
        ecdsa: unsafe { OID_ECDSA_SHA384.as_ptr() },
    },
    HashDef {
        // SHA2-512
        method: HashMethod::empty(),
        block_size: 128u16,
        digest_size: 64u16,
        context_size: size_of::<AnyHashState>() as u16,
        hash_alg: TpmAlgId::Sha2_512,
        oid: unsafe { OID_SHA512.as_ptr() },
        pkcs1: unsafe { OID_PKCS1_SHA512.as_ptr() },
        ecdsa: unsafe { OID_ECDSA_SHA512.as_ptr() },
    },
    HashDef {
        // NULL / invalid
        method: HashMethod::empty(),
        block_size: 0,
        digest_size: 0,
        context_size: 0,
        hash_alg: TpmAlgId::Error,
        oid: core::ptr::null(),
        pkcs1: core::ptr::null(),
        ecdsa: core::ptr::null(),
    },
];

impl TpmHash for NullCrypto {
    fn hash_subsystem_init(&self) -> bool {
        true
    }
    fn hash_subsystem_startup(&self) -> bool {
        true
    }
    fn hash_start(&self, state: &mut HashState, alg: TpmAlgId) -> u16 {
        let ctx = match alg {
            TpmAlgId::Sha2_256 => u32::from(self.client.init(&Sha2_256).expect("sha256_init")),
            TpmAlgId::Sha2_384 => u32::from(self.client.init(&Sha2_384).expect("sha256_init")),
            TpmAlgId::Sha2_512 => u32::from(self.client.init(&Sha2_512).expect("sha256_init")),
            _ => return 0,
        };
        let def = self.hash_def(alg);
        state.hash_alg = alg;
        state.def = def;
        state.ctx_type = HashStateType::Hash;
        state.state = ctx as usize;
        def.digest_size
    }

    fn hash_update(&self, state: &mut HashState, data: &[u8]) {
        let ctx = state.state as u32;
        match (state.ctx_type, state.hash_alg) {
            (HashStateType::Hash, TpmAlgId::Sha2_256) => {
                self.client
                    .update(&Sha2Context::<Sha2_256>::from(ctx), data)
                    .expect("sha256_update");
            }
            (HashStateType::Hash, TpmAlgId::Sha2_384) => {
                self.client
                    .update(&Sha2Context::<Sha2_384>::from(ctx), data)
                    .expect("sha384_update");
            }
            (HashStateType::Hash, TpmAlgId::Sha2_512) => {
                self.client
                    .update(&Sha2Context::<Sha2_512>::from(ctx), data)
                    .expect("sha512_update");
            }
            (HashStateType::Hmac, TpmAlgId::Sha2_256) => {
                self.client
                    .hmac_update(&HmacContext::<HmacSha256>::from(ctx), data)
                    .expect("hmac256_update");
            }
            (HashStateType::Hmac, TpmAlgId::Sha2_384) => {
                self.client
                    .hmac_update(&HmacContext::<HmacSha384>::from(ctx), data)
                    .expect("hmac384_update");
            }
            (HashStateType::Hmac, TpmAlgId::Sha2_512) => {
                self.client
                    .hmac_update(&HmacContext::<HmacSha512>::from(ctx), data)
                    .expect("hmac512_update");
            }
            _ => {}
        }
    }
    fn hash_end(&self, state: &mut HashState, output: &mut [u8]) -> u16 {
        let ctx = state.state as u32;
        let outlen = match state.hash_alg {
            TpmAlgId::Sha2_256 => {
                let digest = self
                    .client
                    .finalize(Sha2Context::<Sha2_256>::from(ctx))
                    .expect("sha256_final");
                min_copy(output, digest.digest())
            }
            TpmAlgId::Sha2_384 => {
                let digest = self
                    .client
                    .finalize(Sha2Context::<Sha2_384>::from(ctx))
                    .expect("sha384_final");
                min_copy(output, digest.digest())
            }
            TpmAlgId::Sha2_512 => {
                let digest = self
                    .client
                    .finalize(Sha2Context::<Sha2_512>::from(ctx))
                    .expect("sha512_final");
                min_copy(output, digest.digest())
            }
            _ => return 0,
        };
        state.ctx_type = HashStateType::Empty;
        state.state = 0;
        outlen as u16
    }

    fn hmac_start(&self, state: &mut HmacState, alg: TpmAlgId, key: &[u8]) -> u16 {
        pw_log::info!("hmac_start: key={}", key.len() as usize);
        let ctx = match alg {
            TpmAlgId::Sha2_256 => u32::from(
                self.client
                    .hmac_init(&HmacSha256, key)
                    .expect("hmac256_init"),
            ),
            TpmAlgId::Sha2_384 => u32::from(
                self.client
                    .hmac_init(&HmacSha384, key)
                    .expect("hmac384_init"),
            ),
            TpmAlgId::Sha2_512 => u32::from(
                self.client
                    .hmac_init(&HmacSha512, key)
                    .expect("hmac512_init"),
            ),
            _ => return 0,
        };
        let def = self.hash_def(alg);
        state.hash_state.hash_alg = alg;
        state.hash_state.def = def;
        state.hash_state.ctx_type = HashStateType::Hmac;
        state.hash_state.state = ctx as usize;
        def.digest_size
    }

    fn hmac_end(&self, state: &mut HmacState, output: &mut [u8]) -> u16 {
        let ctx = state.hash_state.state as u32;
        let outlen = match state.hash_state.hash_alg {
            TpmAlgId::Sha2_256 => {
                let digest = self
                    .client
                    .hmac_finalize(HmacContext::<HmacSha256>::from(ctx))
                    .expect("hmac256_final");
                min_copy(output, digest.digest())
            }
            TpmAlgId::Sha2_384 => {
                let digest = self
                    .client
                    .hmac_finalize(HmacContext::<HmacSha384>::from(ctx))
                    .expect("hmac384_final");
                min_copy(output, digest.digest())
            }
            TpmAlgId::Sha2_512 => {
                let digest = self
                    .client
                    .hmac_finalize(HmacContext::<HmacSha512>::from(ctx))
                    .expect("hmac512_final");
                min_copy(output, digest.digest())
            }
            _ => return 0,
        };
        state.hash_state.ctx_type = HashStateType::Empty;
        state.hash_state.state = 0;
        outlen as u16
    }

    fn hash_def(&self, alg: TpmAlgId) -> &'static HashDef {
        match alg {
            TpmAlgId::Sha2_256 => &HASH_DEFINITIONS[0],
            TpmAlgId::Sha2_384 => &HASH_DEFINITIONS[1],
            TpmAlgId::Sha2_512 => &HASH_DEFINITIONS[2],
            _ => &HASH_DEFINITIONS[3],
        }
    }
    fn hash_by_index(&self, index: usize) -> TpmAlgId {
        if index < HASH_ALGS {
            HASH_DEFINITIONS[index].hash_alg
        } else {
            TpmAlgId::Null
        }
    }
    fn hash_context_alg(&self, state: &HashState) -> TpmAlgId {
        state.hash_alg
    }
    fn hash_export_state(&self, _state: &HashState, _external_state: &mut ExportHashState) {}
    fn hash_import_state(&self, _state: &mut HashState, _external_state: &ExportHashState) {}
}

implement_tpm_hash!(NullCrypto);

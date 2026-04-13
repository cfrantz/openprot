use crypto::{
    ecc::TpmEcc, hash::TpmHash, implement_tpm_ecc, implement_tpm_hash,
    implement_tpm_rsa, implement_tpm_symmetric, rsa::TpmRsa, sym::TpmSymmetric,
    TpmCrypto,
};
use tpm_types::*;

pub mod rand;

pub struct NullCrypto;

impl TpmCrypto for NullCrypto {
    fn get_instance() -> &'static Self {
        &NullCrypto
    }
}

impl TpmEcc for NullCrypto {
    fn ecc_subsystem_init(&self) -> bool {
        true
    }
    fn ecc_subsystem_startup(&self) -> bool {
        true
    }
    fn ecc_parameters_by_index(&self, _index: usize) -> Option<&'static TpmEccCurveMetadata> {
        None
    }
    fn ecc_is_valid_private_key(&self, _d: &[u8], _curve_id: TpmEccCurve) -> bool {
        true
    }
    fn ecc_new_key_pair(
        &self,
        _qout: &mut TpmsEccPoint,
        _dout: &mut Tpm2BEccParameter,
        _curve_id: TpmEccCurve,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn ecc_is_point_on_curve(&self, _curve_id: TpmEccCurve, _q: &TpmsEccPoint) -> bool {
        true
    }
    fn ecc_generate_key(
        &self,
        _public_area: &mut TpmtPublic,
        _sensitive: &mut TpmtSensitive,
        _rand: Option<&mut RandState>,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn ecc_sign(
        &self,
        _signature: &mut TpmtSignature,
        _sign_key: &TpmObject,
        _digest: &[u8],
        _scheme: &TpmtEccScheme,
        _rand: Option<&mut RandState>,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn ecc_verify(
        &self,
        _signature: &TpmtSignature,
        _sign_key: &TpmObject,
        _digest: &[u8],
    ) -> TpmRc {
        TpmRc::Success
    }
    fn ecc_point_multiply(
        &self,
        _rout: &mut TpmsEccPoint,
        _curve_id: TpmEccCurve,
        _p: Option<&TpmsEccPoint>,
        _d: &[u8],
        _q: Option<&TpmsEccPoint>,
        _u: &[u8],
    ) -> TpmRc {
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
        TpmRc::Success
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
        TpmRc::Success
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
        TpmRc::Success
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
        TpmRc::Success
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
        TpmRc::Success
    }
}

static NULL_HASH_DEF: HashDef = HashDef {
    method: HashMethod::empty(),
    block_size: 0,
    digest_size: 0,
    context_size: 0,
    hash_alg: TpmAlgId::Null,
    oid: core::ptr::null(),
    pkcs1: core::ptr::null(),
    ecdsa: core::ptr::null(),
};

impl TpmHash for NullCrypto {
    fn hash_subsystem_init(&self) -> bool {
        true
    }
    fn hash_subsystem_startup(&self) -> bool {
        true
    }
    fn hash_start(&self, _state: &mut HashState, _alg: TpmAlgId) -> u16 {
        0
    }
    fn hash_update(&self, _state: &mut HashState, _data: &[u8]) {}
    fn hash_end(&self, _state: &mut HashState, _output: &mut [u8]) -> u16 {
        0
    }
    fn hmac_start(&self, _state: &mut HmacState, _alg: TpmAlgId, _key: &[u8]) -> u16 {
        0
    }
    fn hmac_end(&self, _state: &mut HmacState, _output: &mut [u8]) -> u16 {
        0
    }
    fn hash_def(&self, _alg: TpmAlgId) -> &'static HashDef {
        &NULL_HASH_DEF
    }
    fn hash_by_index(&self, _index: usize) -> TpmAlgId {
        TpmAlgId::Null
    }
    fn hash_context_alg(&self, _state: &HashState) -> TpmAlgId {
        TpmAlgId::Null
    }
    fn hash_export_state(&self, _state: &HashState, _external_state: &mut ExportHashState) {}
    fn hash_import_state(&self, _state: &mut HashState, _external_state: &ExportHashState) {}
}

impl TpmRsa for NullCrypto {
    fn rsa_subsystem_init(&self) -> bool {
        true
    }
    fn rsa_subsystem_startup(&self) -> bool {
        true
    }
    fn rsa_generate_key(
        &self,
        _public_area: &mut TpmtPublic,
        _sensitive: &mut TpmtSensitive,
        _rand: Option<&mut RandState>,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn rsa_import_key(
        &self,
        _public_area: &mut TpmtPublic,
        _sensitive: &mut TpmtSensitive,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn rsa_encrypt(
        &self,
        _c_out: &mut Tpm2B,
        _d_in: &[u8],
        _key: &TpmObject,
        _scheme: &TpmtRsaDecrypt,
        _label: Option<&[u8]>,
        _rand: Option<&mut RandState>,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn rsa_decrypt(
        &self,
        _d_out: &mut Tpm2B,
        _c_in: &[u8],
        _key: &TpmObject,
        _scheme: &TpmtRsaDecrypt,
        _label: Option<&[u8]>,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn rsa_sign(
        &self,
        _sig_out: &mut TpmtSignature,
        _key: &TpmObject,
        _digest: &[u8],
        _rand: Option<&mut RandState>,
    ) -> TpmRc {
        TpmRc::Success
    }
    fn rsa_verify(&self, _sig: &TpmtSignature, _key: &TpmObject, _digest: &[u8]) -> TpmRc {
        TpmRc::Success
    }
}

impl TpmSymmetric for NullCrypto {
    fn symmetric_subsystem_init(&self) -> bool {
        true
    }
    fn symmetric_subsystem_startup(&self) -> bool {
        true
    }
    fn symmetric_get_block_size(&self, _alg: TpmAlgId, _key_size_in_bits: u16) -> usize {
        0
    }
    fn symmetric_key_validate(&self, _sym_def: &TpmtSymDefObject, _key: &[u8]) -> TpmRc {
        TpmRc::Success
    }
    fn symmetric_encrypt(
        &self,
        _d_out: &mut [u8],
        _algorithm: TpmAlgId,
        _key_size_in_bits: u16,
        _key: &[u8],
        _iv: *mut Tpm2B,
        _mode: TpmAlgId,
        _d_in: &[u8],
    ) -> TpmRc {
        TpmRc::Success
    }
    fn symmetric_decrypt(
        &self,
        _d_out: &mut [u8],
        _algorithm: TpmAlgId,
        _key_size_in_bits: u16,
        _key: &[u8],
        _iv: *mut Tpm2B,
        _mode: TpmAlgId,
        _d_in: &[u8],
    ) -> TpmRc {
        TpmRc::Success
    }
}

implement_tpm_ecc!(NullCrypto);
implement_tpm_hash!(NullCrypto);
implement_tpm_rsa!(NullCrypto);
implement_tpm_symmetric!(NullCrypto);

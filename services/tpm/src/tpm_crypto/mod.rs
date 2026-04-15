use crypto::{ecc::TpmEcc, implement_tpm_ecc, implement_tpm_rsa, rsa::TpmRsa, TpmCrypto};
use crypto_client::backend::CryptoClient;
use tpm_types::*;

pub mod hash;
pub mod rand;
pub mod sym;

pub struct NullCrypto {
    client: CryptoClient,
}

static mut INSTANCE: NullCrypto = NullCrypto {
    client: CryptoClient::new(0),
};

impl NullCrypto {
    pub fn initialize(client: CryptoClient) {
        unsafe {
            INSTANCE.client = client;
        }
    }
}

impl TpmCrypto for NullCrypto {
    fn get_instance() -> &'static Self {
        #[allow(static_mut_refs)]
        unsafe {
            &INSTANCE
        }
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

implement_tpm_ecc!(NullCrypto);
implement_tpm_rsa!(NullCrypto);

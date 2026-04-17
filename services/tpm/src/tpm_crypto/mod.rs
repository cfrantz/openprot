use crypto::{implement_tpm_rsa, rsa::TpmRsa, TpmCrypto};
use crypto_client::backend::CryptoClient;
use tpm_types::*;

pub mod ecc;
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

implement_tpm_rsa!(NullCrypto);

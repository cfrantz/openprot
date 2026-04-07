use otbn_bs::*;
use pw_status::{Error, Result};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

// CDI_1 (Owner) attestation key diverisfier constants.
const CDI1_DIVERSIFIER: sc_keymgr_diversification_t = sc_keymgr_diversification_t {
    salt: [
        0x2d12c2e3, 0x6acc6876, 0x4bfb07ee, 0xc45fc414, 0x5d4fa9de, 0xf295b128, 0x50f49882,
        0xbbdefa29,
    ],
    version: 0,
};

const CDI1_KEY: sc_keymgr_ecc_key_t = sc_keymgr_ecc_key_t {
    type_: sc_keymgr_key_type_kScKeymgrKeyTypeAttestation,
    keygen_seed_idx: 2, //kFlashInfoFieldCdi1KeySeedIdx,
    keymgr_diversifier: &CDI1_DIVERSIFIER,
    required_keymgr_state: sc_keymgr_state_kScKeymgrStateOwnerKey,
};

#[derive(Clone, Default, IntoBytes, Immutable, FromBytes, KnownLayout)]
#[repr(C)]
pub struct EcdsaPublicKey {
    pub x: [u8; 32],
    pub y: [u8; 32],
}

#[derive(Clone, Default, IntoBytes, Immutable, FromBytes, KnownLayout)]
#[repr(C)]
pub struct EcdsaSignature {
    pub r: [u8; 32],
    pub s: [u8; 32],
}

#[derive(Clone, Default, IntoBytes, Immutable, FromBytes, KnownLayout)]
#[repr(C)]
pub struct Attestation {
    pub pubkey_id: [u8; 32],
    pub pubkey: EcdsaPublicKey,
    pub signature: EcdsaSignature,
}

pub fn cdi1_attest(message: &[u8]) -> Result<Attestation> {
    let mut attestation = Attestation::default();
    /*
    let mut pubkey_id = hmac_digest_t {
        digest: Default::default();
    };
    let mut pubkey = ecdsa_p256_public_key_t {
        x: Default::default(),
        y: Default::default(),
    };
    */

    unsafe {
        if otbn_boot_app_load() != 0x739 {
            return Err(Error::Internal);
        }
        if otbn_boot_cert_ecc_p256_keygen(
            CDI1_KEY,
            attestation.pubkey_id.as_mut_ptr() as *mut hmac_digest_t,
            attestation.pubkey.x.as_mut_ptr() as *mut ecdsa_p256_public_key_t,
        ) != 0x739
        {
            return Err(Error::Internal);
        }

        // Reverse the key back to little-endian cuz thats what our tooling uses.
        attestation.pubkey.x.reverse();
        attestation.pubkey.y.reverse();

        if otbn_boot_attestation_key_save(
            CDI1_KEY.keygen_seed_idx,
            CDI1_KEY.type_,
            *CDI1_KEY.keymgr_diversifier,
        ) != 0x739
        {
            return Err(Error::Unknown);
        }
        if otbn_boot_attestation_endorse(
            message.as_ptr() as *mut hmac_digest_t,
            attestation.signature.r.as_ptr() as *mut ecdsa_p256_signature_t,
        ) != 0x739
        {
            return Err(Error::Aborted);
        }
    }
    Ok(attestation)
}

use crypto_common::Opcode;
use otcrypto::{
    BlindedKey, CryptoResult, HardenedBool, HashDigest, KeyConfig, KeyMode, KeySecurityLevel,
    LibVersion, UnblindedKey,
};
use otcrypto::{CryptoInterface, OtCrypto};
use pw_status::{Error, Result};
use zerocopy::FromBytes;

/// Return relevant keysizes: (public, private, blinded)
fn keysize(op: Opcode) -> (usize, usize, usize) {
    match op.class() {
        Opcode::CLASS_ECDSA_P256 => (64, 32, 80),
        Opcode::CLASS_DICE_P256 => (64, 32, 25 * 4),
        Opcode::CLASS_ECDH_P256 => (64, 32, 80),
        Opcode::CLASS_ECDSA_P384 => (96, 48, 112),
        Opcode::CLASS_ECDH_P384 => (96, 48, 112),
        Opcode::CLASS_ED25519 => (64, 32, 80),
        Opcode::CLASS_X25519 => (64, 32, 80),
        Opcode::CLASS_RSA2048 => (256, 256, 768),
        Opcode::CLASS_RSA3072 => (384, 384, 1152),
        Opcode::CLASS_RSA4096 => (512, 512, 1536),
        _ => unreachable!(),
    }
}

pub fn key_pair_gen<'a, GEN>(
    op: Opcode,
    req: &mut [u8],
    rsp: &'a mut [u8],
    generator: GEN,
) -> Result<&'a [u8]>
where
    GEN: Fn(&mut BlindedKey, &mut UnblindedKey) -> CryptoResult,
{
    let (pubsz, privsz, blindsz) = keysize(op);

    let (config, req_rest) = KeyConfig::read_from_prefix(req).map_err(|_| Error::Internal)?;
    let hw_backed = config.hw_backed == HardenedBool::True;
    // TODO: verify config.key_length against privsz.
    let len = {
        let (public, rest) = UnblindedKey::mut_from_prefix(rsp).map_err(|_| Error::Internal)?;
        public.key_mode = config.key_mode;
        public.key_length = pubsz as u32;
        let (pub_key_material, rest) = rest.split_at_mut(pubsz);
        let (private, rest) = BlindedKey::mut_from_prefix(rest).map_err(|_| Error::Internal)?;
        let (priv_key_material, _rest) = rest.split_at_mut(blindsz);

        if hw_backed {
            // Hardware backed keys are specified in the request.
            let (version, req_rest) =
                <u32>::ref_from_prefix(req_rest).map_err(|_| Error::Internal)?;
            let (salt, _) = <[u32; 7]>::ref_from_prefix(req_rest).map_err(|_| Error::Internal)?;
            private.config = config;
            private.keyblob_length = privsz as u32;
            priv_key_material.fill(0);
            OtCrypto::hw_backed_key(*version, salt, private.with_key_material(priv_key_material))?;
        } else {
            // Softwares backed keys are generated into the response.
            private.config = config;
            private.keyblob_length = blindsz as u32;
            private.with_key_material(priv_key_material);
        }

        generator(private, public.with_key_material(pub_key_material))?;
        core::mem::size_of::<UnblindedKey>() + pubsz + core::mem::size_of::<BlindedKey>() + blindsz
    };
    Ok(&rsp[..len])
}

pub fn sign<'a, SIGN>(
    op: Opcode,
    req: &mut [u8],
    rsp: &'a mut [u8],
    signer: SIGN,
) -> Result<&'a [u8]>
where
    SIGN: Fn(&BlindedKey, &HashDigest, &mut [u8]) -> CryptoResult,
{
    let (pubsz, privsz, blindsz) = keysize(op);
    let (private, rest) = BlindedKey::mut_from_prefix(req).map_err(|_| Error::Internal)?;
    // TODO: verify private.keyblob_size with blindsz
    // TODO: verify private.config.key_size with privsz
    let (priv_key_material, rest) = rest.split_at(blindsz);
    // TODO: param, but no Param for ECDSA.
    let (message, _rest) =
        HashDigest::ref_from_prefix_with_elems(rest, privsz / 4).map_err(|_| Error::Internal)?;

    let len = {
        let signature = &mut rsp[..pubsz];
        signer(
            private.with_key_material(priv_key_material),
            message,
            signature,
        )?;
        pubsz
    };
    Ok(&rsp[..len])
}

pub fn verify<'a, VERIFY>(
    op: Opcode,
    req: &mut [u8],
    rsp: &'a mut [u8],
    verifier: VERIFY,
) -> Result<&'a [u8]>
where
    VERIFY: Fn(&UnblindedKey, &HashDigest, &[u8], &mut HardenedBool) -> CryptoResult,
{
    let (pubsz, privsz, _blindsz) = keysize(op);
    let (public, rest) = UnblindedKey::mut_from_prefix(req).map_err(|_| Error::Internal)?;
    // TODO: verify public.key_size with pubsz
    let (pub_key_material, rest) = (&rest[..pubsz], &rest[pubsz..]);
    // TODO: param, but no Param for ECDSA.
    let (signature, rest) = (&rest[..pubsz], &rest[pubsz..]);
    let (message, _rest) =
        HashDigest::ref_from_prefix_with_elems(rest, privsz / 4).map_err(|_| Error::Internal)?;
    let len = {
        let (result, _) = HardenedBool::mut_from_prefix(rsp).map_err(|_| Error::Internal)?;
        verifier(
            public.with_key_material(pub_key_material),
            message,
            signature,
            result,
        )?;
        core::mem::size_of::<HardenedBool>()
    };
    Ok(&rsp[..len])
}

pub fn share_secret<'a, AGREEMENT>(
    op: Opcode,
    req: &mut [u8],
    rsp: &'a mut [u8],
    agreement: AGREEMENT,
) -> Result<&'a [u8]>
where
    AGREEMENT: Fn(&BlindedKey, &UnblindedKey, &mut BlindedKey) -> CryptoResult,
{
    let (pubsz, privsz, blindsz) = keysize(op);
    // TODO: verify private.keyblob_size with blindsz
    // TODO: verify private.config.key_size with privsz
    let (secret_key, rest) = BlindedKey::mut_from_prefix(req).map_err(|_| Error::Internal)?;
    let (secret_key_material, rest) = rest.split_at_mut(blindsz);
    let (public_key, rest) = UnblindedKey::mut_from_prefix(rest).map_err(|_| Error::Internal)?;
    let (public_key_material, _rest) = rest.split_at_mut(pubsz);

    let (secret, rest) = BlindedKey::mut_from_prefix(rsp).map_err(|_| Error::Internal)?;
    let (secret_data, _rest) = rest.split_at_mut(blindsz);
    secret.config = KeyConfig {
        version: LibVersion::_1,
        key_mode: KeyMode::EcdsaP256,
        key_length: privsz as u32,
        hw_backed: HardenedBool::False,
        exportable: HardenedBool::True,
        security_level: KeySecurityLevel::Low,
    };
    secret.keyblob_length = blindsz as u32;

    pw_log::info!("key_agreement");
    agreement(
        secret_key.with_key_material(secret_key_material),
        public_key.with_key_material(public_key_material),
        secret.with_key_material(secret_data),
    )?;
    pw_log::info!("key_agreement done");
    let len = core::mem::size_of::<BlindedKey>() + blindsz;
    Ok(&rsp[..len])
}

use crypto_common::{CipherMode, Opcode};
//use otcrypto::{BlindedKey, CryptoResult, HardenedBool, HashDigest, KeyConfig, UnblindedKey};
use otcrypto::{AesMode, AesOperation, AesPadding, BlindedKey, CryptoInterface, OtCrypto};
use pw_status::{Error, Result};
use zerocopy::FromBytes;

pub fn aes_encrypt_decrypt<'a>(op: Opcode, req: &mut [u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
    let operation = match op {
        Opcode::AES_ENCRYPT => AesOperation::Encrypt,
        Opcode::AES_DECRYPT => AesOperation::Decrypt,
        _ => return Err(Error::Unknown),
    };
    let (mode, rest) = CipherMode::mut_from_prefix(req).map_err(|_| Error::Internal)?;
    let mode = match *mode {
        CipherMode::ECB => AesMode::Ecb,
        CipherMode::CBC => AesMode::Cbc,
        CipherMode::CFB => AesMode::Cfb,
        CipherMode::CTR => AesMode::Ctr,
        CipherMode::OFB => AesMode::Ofb,
        _ => return Err(Error::Unknown),
    };
    let (key, rest) = BlindedKey::mut_from_prefix(rest).map_err(|_| Error::Internal)?;
    // TODO: verify private.keyblob_size with blindsz
    // TODO: verify private.config.key_size with privsz
    let (key_material, rest) = rest.split_at_mut(key.keyblob_length as usize);
    let (iv_in, rest) = <[u8; 16]>::mut_from_prefix(rest).map_err(|_| Error::Internal)?;
    let input = rest;

    let (iv_out, rest) = rsp.split_at_mut(16);
    let (output, _rest) = rest.split_at_mut(input.len());

    pw_log::info!("Sending AES");

    let mut input_len = input.len();
    let len = input_len & !15;
    if input_len >= 16 {
        OtCrypto::aes(
            key.with_key_material(key_material),
            iv_in,
            mode,
            operation,
            &input[..len],
            AesPadding::Null,
            &mut output[..len],
        )?;

        input_len -= len;
    }
    if input_len > 0 {
        pw_log::info!("AES remainder: {} bytes", input_len as usize);
        let mut i2 = [0u8; 16];
        let mut o2 = [0u8; 16];
        i2[..input_len].copy_from_slice(&input[len..]);
        OtCrypto::aes(
            key.with_key_material(key_material),
            iv_in,
            mode,
            operation,
            &i2,
            AesPadding::Null,
            &mut o2,
        )?;
        output[len..].copy_from_slice(&o2[..input_len]);
    }

    pw_log::info!("AES done");

    // Copy the IV back to the output.
    iv_out.copy_from_slice(iv_in);
    Ok(&rsp[..(16 + input.len())])
}

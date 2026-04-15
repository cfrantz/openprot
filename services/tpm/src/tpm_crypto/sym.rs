use crypto::{implement_tpm_symmetric, sym::TpmSymmetric};
use tpm_types::*;
//use crypto_client::backend::CryptoClient;
use crypto_common::symmetric_key::{Aes128Key, Aes192Key, Aes256Key, KeyMode};
use crypto_common::CipherMode;
use crypto_traits::symmetric::SymmetricOp;

fn get_modes(mode: TpmAlgId) -> Result<(KeyMode, CipherMode), ()> {
    match mode {
        TpmAlgId::Ecb => Ok((KeyMode::AesEcb, CipherMode::ECB)),
        TpmAlgId::Cfb => Ok((KeyMode::AesCfb, CipherMode::CFB)),
        TpmAlgId::Cbc => Ok((KeyMode::AesCbc, CipherMode::CBC)),
        TpmAlgId::Ctr => Ok((KeyMode::AesCtr, CipherMode::CTR)),
        _ => Err(()),
    }
}

use crate::tpm_crypto::NullCrypto;

const AES_BLOCK_SIZE: usize = 16;

fn check_inputs(
    algorithm: TpmAlgId,
    mode: TpmAlgId,
    key_size: usize,
    data_size: usize,
) -> Result<(), TpmRc> {
    if data_size == 0 {
        // If there's no data, then the result it Success.
        return Err(TpmRc::Success);
    }
    if key_size == 0 || algorithm != TpmAlgId::Aes {
        return Err(TpmRc::Failure);
    }
    match mode {
        TpmAlgId::Ecb => {
            if data_size % AES_BLOCK_SIZE != 0 {
                return Err(TpmRc::Size);
            }
        }
        TpmAlgId::Cbc => {
            if data_size % AES_BLOCK_SIZE != 0 {
                return Err(TpmRc::Size);
            }
        }
        TpmAlgId::Ctr => {}
        TpmAlgId::Cfb => {}
        _ => return Err(TpmRc::Failure),
    }
    Ok(())
}

impl TpmSymmetric for NullCrypto {
    fn symmetric_subsystem_init(&self) -> bool {
        true
    }
    fn symmetric_subsystem_startup(&self) -> bool {
        true
    }

    fn symmetric_get_block_size(&self, alg: TpmAlgId, key_size_in_bits: u16) -> usize {
        match (alg, key_size_in_bits) {
            (TpmAlgId::Aes, 128) => AES_BLOCK_SIZE,
            (TpmAlgId::Aes, 192) => AES_BLOCK_SIZE,
            (TpmAlgId::Aes, 256) => AES_BLOCK_SIZE,
            _ => 0,
        }
    }

    fn symmetric_key_validate(&self, sym_def: &TpmtSymDefObject, key: &[u8]) -> TpmRc {
        // TODO: do a better job validating the key
        if key.len() == (sym_def.key_bits as usize + 7) / 8 {
            TpmRc::Success
        } else {
            TpmRc::KeySize
        }
    }

    fn symmetric_encrypt(
        &self,
        d_out: &mut [u8],
        algorithm: TpmAlgId,
        key_size_in_bits: u16,
        key: &[u8],
        iv: *mut Tpm2B,
        mode: TpmAlgId,
        d_in: &[u8],
    ) -> TpmRc {
        if let Err(e) = check_inputs(algorithm, mode, key.len(), d_out.len()) {
            return e;
        }
        let (km, cm) = match get_modes(mode) {
            Ok((km, cm)) => (km, cm),
            Err(_) => {
                pw_log::error!("invalid encrypt mode {:x}", mode.0 as u16);
                return TpmRc::Failure;
            }
        };
        // If the iv is provided, then it is expected to be block sized. In some
        // cases, the caller is providing an array of 0's that is equal to
        // [MAX_SYM_BLOCK_SIZE] with no knowledge of the actual block size. This
        // function will set it.
        let mut default_iv = [0u8; AES_BLOCK_SIZE];
        let iv: &mut [u8] = if !iv.is_null() && mode != TpmAlgId::Ecb {
            unsafe {
                // Set correct length as `iv` is in/out
                Tpm2B::set_length(iv, AES_BLOCK_SIZE as u16);
                Tpm2B::as_mut_bytes(iv)
            }
        } else {
            &mut default_iv
        };
        let result = match key_size_in_bits {
            128 => {
                let key = Aes128Key::with_key_material(km, key);
                self.client.encrypt(&cm, &key, iv, d_in, d_out)
            }
            192 => {
                let key = Aes192Key::with_key_material(km, key);
                self.client.encrypt(&cm, &key, iv, d_in, d_out)
            }
            256 => {
                let key = Aes256Key::with_key_material(km, key);
                self.client.encrypt(&cm, &key, iv, d_in, d_out)
            }
            _ => return TpmRc::Failure,
        };
        match result {
            Ok(_) => TpmRc::Success,
            Err(_) => TpmRc::Failure,
        }
    }

    fn symmetric_decrypt(
        &self,
        d_out: &mut [u8],
        algorithm: TpmAlgId,
        key_size_in_bits: u16,
        key: &[u8],
        iv: *mut Tpm2B,
        mode: TpmAlgId,
        d_in: &[u8],
    ) -> TpmRc {
        if let Err(e) = check_inputs(algorithm, mode, key.len(), d_out.len()) {
            return e;
        }
        let (km, cm) = match get_modes(mode) {
            Ok((km, cm)) => (km, cm),
            Err(_) => {
                pw_log::error!("invalid decrypt mode {:x}", mode.0 as u16);
                return TpmRc::Failure;
            }
        };
        // If the iv is provided, then it is expected to be block sized. In some
        // cases, the caller is providing an array of 0's that is equal to
        // [MAX_SYM_BLOCK_SIZE] with no knowledge of the actual block size. This
        // function will set it.
        let mut default_iv = [0u8; AES_BLOCK_SIZE];
        let iv: &mut [u8] = if !iv.is_null() && mode != TpmAlgId::Ecb {
            unsafe {
                // Set correct length as `iv` is in/out
                Tpm2B::set_length(iv, AES_BLOCK_SIZE as u16);
                Tpm2B::as_mut_bytes(iv)
            }
        } else {
            &mut default_iv
        };
        let result = match key_size_in_bits {
            128 => {
                let key = Aes128Key::with_key_material(km, key);
                self.client.decrypt(&cm, &key, iv, d_in, d_out)
            }
            192 => {
                let key = Aes192Key::with_key_material(km, key);
                self.client.decrypt(&cm, &key, iv, d_in, d_out)
            }
            256 => {
                let key = Aes256Key::with_key_material(km, key);
                self.client.decrypt(&cm, &key, iv, d_in, d_out)
            }
            _ => return TpmRc::Failure,
        };
        match result {
            Ok(_) => TpmRc::Success,
            Err(_) => TpmRc::Failure,
        }
    }
}

implement_tpm_symmetric!(NullCrypto);

use otcrypto::{CryptoInterface, OtCrypto};
use pw_status::Result;

pub fn instantiate<'a>(_req: &[u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
    OtCrypto::drbg_instantiate(_req)?;
    Ok(&rsp[0..0])
}

pub fn reseed<'a>(_req: &[u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
    OtCrypto::drbg_reseed(_req)?;
    Ok(&rsp[0..0])
}

pub fn generate<'a>(_req: &[u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
    OtCrypto::drbg_generate(_req, rsp)?;
    Ok(rsp)
}

pub fn uninstantiate<'a>(_req: &[u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
    OtCrypto::drbg_uninstantiate()?;
    Ok(&rsp[0..0])
}

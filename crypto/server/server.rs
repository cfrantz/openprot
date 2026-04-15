use crate::asymmetric;
use crate::digest::Sha2Contexts;
use crate::drbg;
use crate::hmac::HmacContexts;
use crate::symmetric;
use crypto_common::Opcode;
use pw_status::Error;
use zerocopy::FromBytes;

use otcrypto::{CryptoInterface, OtCrypto};
//use util::hexdump;

#[derive(Default)]
pub struct Server {
    digest: Sha2Contexts<4>,
    hmac: HmacContexts<4>,
}

impl Server {
    pub fn exec<'a>(&mut self, req: &mut [u8], rsp: &'a mut [u8]) -> Result<&'a [u8], Error> {
        pw_log::info!("request:");
        util_misc::hexdump(req);
        let (&mut opcode, req) = Opcode::mut_from_prefix(req).map_err(|_| Error::Internal)?;
        match opcode {
            Opcode::AES_ENCRYPT => symmetric::aes_encrypt_decrypt(opcode, req, rsp),
            Opcode::AES_DECRYPT => symmetric::aes_encrypt_decrypt(opcode, req, rsp),
            Opcode::ECDSA_P256_KEYGEN => {
                asymmetric::key_pair_gen(opcode, req, rsp, OtCrypto::ecdsa_p256_keygen)
            }
            Opcode::ECDSA_P256_SIGN => {
                asymmetric::sign(opcode, req, rsp, OtCrypto::ecdsa_p256_sign)
            }
            Opcode::ECDSA_P256_VERIFY => {
                asymmetric::verify(opcode, req, rsp, OtCrypto::ecdsa_p256_verify)
            }

            Opcode::DICE_P256_KEYGEN => {
                asymmetric::key_pair_gen(opcode, req, rsp, OtCrypto::dice_p256_keygen)
            }
            Opcode::DICE_P256_SIGN => asymmetric::sign(opcode, req, rsp, OtCrypto::dice_p256_sign),

            //Opcode::ECDH_P256_KEYGEN => Err(Error::Unimplemented),
            //Opcode::ECDH_P256_KEY_AGREEMENT => Err(Error::Unimplemented),

            //Opcode::ECDSA_P384_KEYGEN => p384::Ops::key_pair_gen(req, rsp),
            //Opcode::ECDSA_P384_SIGN => p384::Ops::sign(req, rsp),
            //Opcode::ECDSA_P384_VERIFY => p384::Ops::verify(req, rsp),
            //Opcode::ECDH_P384_KEYGEN => Err(Error::Unimplemented),
            //Opcode::ECDH_P384_KEY_AGREEMENT => Err(Error::Unimplemented),

            //Opcode::ED25519_KEYGEN => Err(Error::Unimplemented),
            //Opcode::ED25519_SIGN => Err(Error::Unimplemented),
            //Opcode::ED25519_VERIFY => Err(Error::Unimplemented),
            //Opcode::X25519_KEYGEN => Err(Error::Unimplemented),
            //Opcode::X25519_KEY_AGREEMENT => Err(Error::Unimplemented),

            //Opcode::RSA2048_KEYGEN => rsa2048::Ops::key_pair_gen(req, rsp),
            //Opcode::RSA2048_SIGN =>   rsa2048::Ops::sign(req, rsp),
            //Opcode::RSA2048_VERIFY => rsa2048::Ops::verify(req, rsp),
            //Opcode::RSA3072_KEYGEN => rsa3072::Ops::key_pair_gen(req, rsp),
            //Opcode::RSA3072_SIGN =>   rsa3072::Ops::sign(req, rsp),
            //Opcode::RSA3072_VERIFY => rsa3072::Ops::verify(req, rsp),
            //Opcode::RSA4096_KEYGEN => rsa4096::Ops::key_pair_gen(req, rsp),
            //Opcode::RSA4096_SIGN =>   rsa4096::Ops::sign(req, rsp),
            //Opcode::RSA4096_VERIFY => rsa4096::Ops::verify(req, rsp),
            Opcode::SHA2_256_INIT => self.digest.init(opcode, req, rsp),
            Opcode::SHA2_256_UPDATE => self.digest.update(opcode, req, rsp),
            Opcode::SHA2_256_FINAL => self.digest.finalize(opcode, req, rsp),
            Opcode::SHA2_512_INIT => self.digest.init(opcode, req, rsp),
            Opcode::SHA2_512_UPDATE => self.digest.update(opcode, req, rsp),
            Opcode::SHA2_512_FINAL => self.digest.finalize(opcode, req, rsp),
            Opcode::DRBG_INSTANTIATE => drbg::instantiate(req, rsp),
            Opcode::DRBG_RESEED => drbg::reseed(req, rsp),
            Opcode::DRBG_GENERATE => drbg::generate(req, rsp),
            Opcode::DRBG_UNINSTANTIATE => drbg::uninstantiate(req, rsp),
            Opcode::HMAC_SHA2_256_INIT
            | Opcode::HMAC_SHA2_384_INIT
            | Opcode::HMAC_SHA2_512_INIT => self.hmac.init(opcode, req, rsp),
            Opcode::HMAC_SHA2_256_UPDATE
            | Opcode::HMAC_SHA2_384_UPDATE
            | Opcode::HMAC_SHA2_512_UPDATE => self.hmac.update(opcode, req, rsp),
            Opcode::HMAC_SHA2_256_FINAL
            | Opcode::HMAC_SHA2_384_FINAL
            | Opcode::HMAC_SHA2_512_FINAL => self.hmac.finalize(opcode, req, rsp),
            _ => {
                pw_log::info!("Got opcode {}", opcode.as_str() as &str);
                Err(Error::Unknown)
            }
        }
    }
}

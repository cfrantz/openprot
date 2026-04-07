use crate::backend::CryptoClient;
use crate::util;
use crypto_common::Opcode;
use crypto_traits::drbg::Drbg;

impl Drbg for CryptoClient {
    fn instantiate(&self, personalization_string: &[u8]) -> Result<(), Self::Error> {
        util::drbg::instantiate(self, Opcode::DRBG_INSTANTIATE, personalization_string)
    }

    fn reseed(&self, additional_input: &[u8]) -> Result<(), Self::Error> {
        util::drbg::reseed(self, Opcode::DRBG_RESEED, additional_input)
    }

    fn generate(&self, additional_input: &[u8], output: &mut [u8]) -> Result<(), Self::Error> {
        util::drbg::generate(self, Opcode::DRBG_GENERATE, additional_input, output)
    }

    fn uninstantiate(&self) -> Result<(), Self::Error> {
        util::drbg::uninstantiate(self, Opcode::DRBG_UNINSTANTIATE)
    }
}

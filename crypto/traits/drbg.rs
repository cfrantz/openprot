use crate::error::ErrorType;

pub trait Drbg: ErrorType {
    fn instantiate(&self, personalization_string: &[u8]) -> Result<(), Self::Error>;
    fn reseed(&self, additional_input: &[u8]) -> Result<(), Self::Error>;
    fn generate(&self, additional_input: &[u8], output: &mut [u8]) -> Result<(), Self::Error>;
    fn uninstantiate(&self) -> Result<(), Self::Error>;
}

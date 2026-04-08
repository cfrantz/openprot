use crypto_traits::backend::Backend;
use crypto_traits::error::ErrorType;

pub struct CryptoClient {
    pub ipc: u32,
}

impl ErrorType for CryptoClient {
    type Error = pw_status::Error;
}
impl Backend for CryptoClient {}

impl CryptoClient {
    pub const fn new(ipc: u32) -> Self {
        CryptoClient { ipc }
    }
}

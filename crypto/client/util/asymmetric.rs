use crate::backend::CryptoClient;
use crate::util;
use crypto_common::Opcode;

use otcrypto::{HardenedBool, KeyConfig};
use pw_status::Error;
use userspace::time::Instant;
use zerocopy::IntoBytes;

pub(crate) fn keygen(
    client: &CryptoClient,
    op: Opcode,
    config: &KeyConfig,
    private_key: &mut [u8],
    public_key: &mut [u8],
) -> Result<(), Error> {
    let mut status = 0u32;
    if config.hw_backed == HardenedBool::True {
        // For hw_backed keys, we send the blinded private material in
        // and get only a public key back.
        let _ = util_ipc::transaction::<{ util::SIZE }>(
            client.ipc,
            &[/*op=*/ op.as_bytes(), /*config=*/ private_key],
            &mut [status.as_mut_bytes(), public_key],
            Instant::MAX,
        )?;
    } else {
        // For non-hw_backed keys, we send the key config in and get
        // back both private and public keys.
        let _ = util_ipc::transaction::<{ util::SIZE }>(
            client.ipc,
            &[
                /*op=*/ op.as_bytes(),
                /*config=*/ config.as_bytes(),
            ],
            &mut [status.as_mut_bytes(), public_key, private_key],
            Instant::MAX,
        )?;
    }
    util_ipc::check_status_code(status)?;
    Ok(())
}

pub(crate) fn sign(
    client: &CryptoClient,
    op: Opcode,
    key: &[u8],
    param: &[u8],
    message: &[u8],
    signature: &mut [u8],
) -> Result<(), Error>
where
{
    let mut status = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*key=*/ key,
            /*param=*/ param,
            /*message=*/ message,
        ],
        &mut [status.as_mut_bytes(), signature],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

pub(crate) fn verify(
    client: &CryptoClient,
    op: Opcode,
    key: &[u8],
    param: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, Error> {
    let mut status = 0u32;
    let mut result = HardenedBool::False;

    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*key=*/ key,
            /*param=*/ param,
            /*signature=*/ signature,
            /*message=*/ message,
        ],
        &mut [status.as_mut_bytes(), result.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(result == HardenedBool::True)
}

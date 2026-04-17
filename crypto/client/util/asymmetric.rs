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
    salt: Option<&[u8]>,
    private_key: &mut [u8],
    public_key: &mut [u8],
) -> Result<(), Error> {
    let mut status = 0u32;
    if config.hw_backed == HardenedBool::True {
        let Some(salt) = salt else {
            return Err(Error::InvalidArgument);
        };
        // For hw_backed keys, we send the key config and salt in and
        // get back a public key and blinded key setup with the proper
        // hardware initializer.
        let _ = util_ipc::transaction::<{ util::SIZE }>(
            client.ipc,
            &[
                /*op=*/ op.as_bytes(),
                /*config=*/ config.as_bytes(),
                /*salt=*/ salt,
            ],
            &mut [status.as_mut_bytes(), public_key, private_key],
            Instant::MAX,
        )?;
    } else {
        // For non-hw_backed keys, we send the key config in and get
        // back both public and private keys.
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

pub(crate) fn share_secret(
    client: &CryptoClient,
    op: Opcode,
    secret_key: &[u8],
    public_key: &[u8],
    secret: &mut [u8],
) -> Result<(), Error> {
    let mut status = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*secret_key=*/ secret_key,
            /*public_key=*/ public_key,
        ],
        &mut [status.as_mut_bytes(), secret],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

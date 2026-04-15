use crate::backend::CryptoClient;
use crate::util;
use crypto_common::Opcode;

use pw_status::Error;
use userspace::time::Instant;
use zerocopy::IntoBytes;

pub(crate) fn instantiate(client: &CryptoClient, op: Opcode, perso: &[u8]) -> Result<(), Error> {
    let mut status = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[/*op=*/ op.as_bytes(), /*perso=*/ perso],
        &mut [status.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

pub(crate) fn reseed(
    client: &CryptoClient,
    op: Opcode,
    additional_input: &[u8],
) -> Result<(), Error> {
    let mut status = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*additional_input=*/ additional_input,
        ],
        &mut [status.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

pub(crate) fn generate(
    client: &CryptoClient,
    op: Opcode,
    additional_input: &[u8],
    output: &mut [u8],
) -> Result<(), Error> {
    let mut status = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*additional_input=*/ additional_input,
        ],
        &mut [status.as_mut_bytes(), output.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

pub(crate) fn uninstantiate(client: &CryptoClient, op: Opcode) -> Result<(), Error> {
    let mut status = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[/*op=*/ op.as_bytes()],
        &mut [status.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

use crate::backend::CryptoClient;
use crate::util;
use crypto_common::Opcode;

use pw_status::Error;
use userspace::time::Instant;
use zerocopy::IntoBytes;

pub(crate) fn init(client: &CryptoClient, op: Opcode, key: &[u8]) -> Result<u32, Error> {
    let mut status = 0u32;
    let mut index = 0u32;
    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[/*op=*/ op.as_bytes(), /*key=*/ key],
        &mut [status.as_mut_bytes(), index.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(index)
}

pub(crate) fn update(
    client: &CryptoClient,
    op: Opcode,
    index: u32,
    data: &[u8],
) -> Result<(), Error> {
    let mut status = 0u32;

    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*index=*/ index.as_bytes(),
            /*message=*/ data,
        ],
        &mut [status.as_mut_bytes()],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

pub(crate) fn finalize(
    client: &CryptoClient,
    op: Opcode,
    index: u32,
    tag: &mut [u8],
) -> Result<(), Error> {
    let mut status = 0u32;

    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[/*op=*/ op.as_bytes(), /*index=*/ index.as_bytes()],
        &mut [status.as_mut_bytes(), tag],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

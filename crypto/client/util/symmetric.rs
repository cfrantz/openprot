use crate::backend::CryptoClient;
use crate::util;
use crypto_common::CipherMode;
use crypto_common::Opcode;

use pw_status::Error;
use userspace::time::Instant;
use zerocopy::IntoBytes;

pub(crate) fn encrypt_decrypt(
    client: &CryptoClient,
    op: Opcode,
    mode: &CipherMode,
    key: &[u8],
    iv: &mut [u8],
    input: &[u8],
    output: &mut [u8],
) -> Result<(), Error> {
    let mut status = 0u32;
    let mut iv_in = [0u8; 16];
    iv_in.copy_from_slice(iv);

    let _ = util_ipc::transaction::<{ util::SIZE }>(
        client.ipc,
        &[
            /*op=*/ op.as_bytes(),
            /*mode=*/ mode.as_bytes(),
            /*key=*/ key,
            /*iv=*/ &iv_in,
            /*input=*/ input,
        ],
        &mut [status.as_mut_bytes(), iv, output],
        Instant::MAX,
    )?;
    util_ipc::check_status_code(status)?;
    Ok(())
}

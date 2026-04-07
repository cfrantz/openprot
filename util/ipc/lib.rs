#![no_std]
use pw_status::{Error, Result};
use userspace::syscall;
use userspace::time::Instant;

pub fn check_status_code(code: u32) -> Result<()> {
    match code {
        0 => Ok(()),
        1 => Err(Error::Cancelled),
        2 => Err(Error::Unknown),
        3 => Err(Error::InvalidArgument),
        4 => Err(Error::DeadlineExceeded),
        5 => Err(Error::NotFound),
        6 => Err(Error::AlreadyExists),
        7 => Err(Error::PermissionDenied),
        8 => Err(Error::ResourceExhausted),
        9 => Err(Error::FailedPrecondition),
        10 => Err(Error::Aborted),
        11 => Err(Error::OutOfRange),
        12 => Err(Error::Unimplemented),
        13 => Err(Error::Internal),
        14 => Err(Error::Unavailable),
        15 => Err(Error::DataLoss),
        16 => Err(Error::Unauthenticated),
        _ => Err(Error::Unknown),
    }
}

/// Perform a pigweed IPC transaction.
///
/// This function provides an iovec-like abstraction until we get iovecs into the pigweed kernel.
pub fn transaction<const N: usize>(
    channel: u32,
    request: &[&[u8]],
    response: &mut [&mut [u8]],
    deadline: Instant,
) -> Result<usize> {
    if false {
        //let _n = N;
        //syscall::channel_transact_iovec(channel, request, response, deadline)
        Err(Error::Unimplemented)
    } else {
        let mut buffer = [0u8; N];
        let mut offset = 0usize;

        for item in request.iter() {
            let sz = offset + item.len();
            buffer[offset..sz].copy_from_slice(item);
            offset = sz;
        }
        let req = unsafe {
            // SAFETY: naughty creation of a const ref to the same slice
            // so we can use the same buffer for send and recv.
            core::slice::from_raw_parts(buffer.as_ptr(), offset)
        };
        let rsplen = syscall::channel_transact(channel, req, &mut buffer, deadline)?;

        offset = 0usize;
        let rsp = &buffer[..rsplen];
        for item in response.iter_mut() {
            let sz = offset + item.len();
            // TODO: how to handle an incomplete response?.
            if sz > rsp.len() {
                break;
            }
            item.copy_from_slice(&rsp[offset..sz]);
            offset = sz;
        }
        Ok(rsplen)
    }
}

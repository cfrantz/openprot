//! Flash IPC client implementation.

#![no_std]
use core::num::NonZero;

use hal_flash::{Flash, FlashAddress};
use services_flash_opcode::FlashResponseHeaderExt as _;
use services_flash_opcode::*;
use userspace::time::Instant;
use util_error::{self as error, ErrorCode};
use util_ipc::IpcChannel;
use util_types::PowerOf2Usize;
use zerocopy::{FromZeros, IntoBytes};

/// This struct implements the `Flash` trait by sending IPC requests to a remote
/// flash server.
pub struct FlashIpcClient {
    ipc: IpcChannel,
    page_size: PowerOf2Usize,
    total_size: NonZero<usize>,
    erasable_sizes_bitmap: u32,
}

impl FlashIpcClient {
    /// Creates a new `FlashIpcClient` from an existing IPC channel.
    ///
    /// This constructor will perform an IPC transaction to retrieve flash
    /// information (page size and total size) from the server.
    pub fn new(ipc: IpcChannel) -> Result<Self, ErrorCode> {
        let req_hdr = FlashRequestHeader::new(IPC_OP_FLASH_GET_INFO, 0);
        let mut rsp_hdr = FlashResponseHeader::success(0, 0);
        let mut info = FlashInfo::new_zeroed();

        ipc.transaction::<64>(
            &[req_hdr.as_bytes()],
            &mut [rsp_hdr.as_mut_bytes(), info.as_mut_bytes()],
            Instant::MAX,
        )?;
        IpcChannel::check_status(rsp_hdr.status)?;
        if rsp_hdr.payload_length() < core::mem::size_of::<FlashInfo>() {
            return Err(error::IPC_ERROR_RSP_BAD_LEN);
        }

        let Some(page_size) = PowerOf2Usize::new(info.page_size as usize) else {
            return Err(error::FLASH_GENERIC_INVALID_PAGE_SIZE);
        };
        let Some(total_size) = NonZero::new(info.total_size as usize) else {
            return Err(error::FLASH_GENERIC_INVALID_SIZE);
        };
        Ok(Self {
            ipc,
            page_size,
            total_size,
            erasable_sizes_bitmap: info.erasable_sizes_bitmap,
        })
    }
}

impl Flash for FlashIpcClient {
    fn geometry(&self) -> (NonZero<usize>, PowerOf2Usize, u32) {
        (self.total_size, self.page_size, self.erasable_sizes_bitmap)
    }

    fn erase(&mut self, start_addr: FlashAddress, size: PowerOf2Usize) -> Result<(), ErrorCode> {
        let payload = FlashEraseRequest {
            addr: start_addr.into(),
            size: size.get() as u32,
        };
        let req_hdr = FlashRequestHeader::new(
            IPC_OP_FLASH_ERASE,
            core::mem::size_of::<FlashEraseRequest>(),
        );
        let mut rsp_hdr = FlashResponseHeader::success(0, 0);
        self.ipc.transaction::<64>(
            &[req_hdr.as_bytes(), payload.as_bytes()],
            &mut [rsp_hdr.as_mut_bytes()],
            Instant::MAX,
        )?;
        IpcChannel::check_status(rsp_hdr.status)
    }

    fn program(&mut self, start_addr: FlashAddress, data: &[u8]) -> Result<(), ErrorCode> {
        if data.len() > FLASH_IPC_MAX_DATA_LEN {
            return Err(error::IPC_ERROR_BAD_REQ_LEN);
        }
        let payload_prefix = FlashProgramRequest { addr: start_addr.into() };
        let req_payload_len = core::mem::size_of::<FlashProgramRequest>() + data.len();
        let req_hdr = FlashRequestHeader::new(IPC_OP_FLASH_PROGRAM, req_payload_len);
        let mut rsp_hdr = FlashResponseHeader::success(0, 0);
        self.ipc.transaction::<2064>(
            &[req_hdr.as_bytes(), payload_prefix.as_bytes(), data],
            &mut [rsp_hdr.as_mut_bytes()],
            Instant::MAX,
        )?;
        IpcChannel::check_status(rsp_hdr.status)
    }

    fn read(&mut self, start_addr: FlashAddress, buf: &mut [u8]) -> Result<(), ErrorCode> {
        if buf.len() > FLASH_IPC_MAX_DATA_LEN {
            return Err(error::IPC_ERROR_BAD_REQ_LEN);
        }
        let payload = FlashReadRequest {
            addr: start_addr.into(),
            length: buf.len() as u32,
        };
        let req_hdr = FlashRequestHeader::new(
            IPC_OP_FLASH_READ,
            core::mem::size_of::<FlashReadRequest>(),
        );
        let mut rsp_hdr = FlashResponseHeader::success(0, 0);
        self.ipc.transaction::<2064>(
            &[req_hdr.as_bytes(), payload.as_bytes()],
            &mut [rsp_hdr.as_mut_bytes(), buf],
            Instant::MAX,
        )?;
        IpcChannel::check_status(rsp_hdr.status)?;
        if rsp_hdr.payload_length() != buf.len() {
            return Err(error::IPC_ERROR_RSP_BAD_LEN);
        }
        Ok(())
    }
}

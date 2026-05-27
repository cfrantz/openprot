//! Flash IPC server implementation.

#![no_std]

use hal_flash::{Flash, FlashAddress};
use services_flash_opcode::{FlashResponseHeaderExt as _, *};
use util_error::{self as error, ErrorCode};
use util_ipc::IpcChannel;
use util_types::PowerOf2Usize;
use zerocopy::{FromBytes, IntoBytes};

/// A flash server that handles flash IPC requests.
///
/// This struct wraps an object implementing the `Flash` trait and provides
/// an IPC interface to it.
pub struct FlashIpcServer<TFlash: Flash> {
    flash: TFlash,
}

impl<TFlash: Flash> FlashIpcServer<TFlash> {
    /// Creates a new `FlashIpcServer` wrapping the given flash implementation.
    pub fn new(flash: TFlash) -> Self {
        Self { flash }
    }

    fn handle_get_info<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], ErrorCode> {
        let (info, _rest) =
            FlashInfo::mut_from_prefix(out).map_err(|_| error::IPC_ERROR_BAD_REQ_LEN)?;
        let (total_size, page_size, erasable_sizes_bitmap) = self.flash.geometry();
        info.page_size = page_size.get() as u32;
        info.total_size = total_size.get() as u32;
        info.erasable_sizes_bitmap = erasable_sizes_bitmap;
        Ok(info.as_bytes())
    }

    fn handle_erase(&mut self, payload: &[u8]) -> Result<(), ErrorCode> {
        let (req, _rest) =
            FlashEraseRequest::read_from_prefix(payload).map_err(|_| error::IPC_ERROR_BAD_REQ_LEN)?;
        let Some(size) = PowerOf2Usize::new(req.size as usize) else {
            return Err(error::FLASH_GENERIC_ERASE_INVALID_SIZE);
        };
        self.flash.erase(FlashAddress::from(req.addr), size)
    }

    fn handle_program(&mut self, payload: &[u8]) -> Result<(), ErrorCode> {
        let (req, data) =
            FlashProgramRequest::read_from_prefix(payload).map_err(|_| error::IPC_ERROR_BAD_REQ_LEN)?;
        if data.len() > FLASH_IPC_MAX_DATA_LEN {
            return Err(error::IPC_ERROR_BAD_REQ_LEN);
        }
        self.flash.program(FlashAddress::from(req.addr), data)
    }

    fn handle_read<'a>(&mut self, payload: &[u8], out: &'a mut [u8]) -> Result<&'a [u8], ErrorCode> {
        let (req, _rest) =
            FlashReadRequest::read_from_prefix(payload).map_err(|_| error::IPC_ERROR_BAD_REQ_LEN)?;
        let length = req.length as usize;
        if length > FLASH_IPC_MAX_DATA_LEN {
            return Err(error::IPC_ERROR_BAD_REQ_LEN);
        }
        let out = out.get_mut(..length).ok_or(error::IPC_ERROR_BAD_REQ_LEN)?;
        self.flash.read(FlashAddress::from(req.addr), out)?;
        Ok(out)
    }

    /// Handles a single IPC request.
    ///
    /// This method waits for a request on the given IPC channel, dispatches it
    /// to the appropriate handler, and sends the response.
    pub fn handle_one(&mut self, ipc: &IpcChannel, data: &mut [u8]) -> Result<(), ErrorCode> {
        ipc.wait_readable()?;
        let len = ipc.read(0, data)?;
        let req = &data[..len];

        let mut rsp_hdr = FlashResponseHeader::success(0, 0);
        let mut rsp_len = FlashResponseHeader::SIZE;

        let outcome = (|| -> Result<(), ErrorCode> {
            let (hdr, rest) = FlashRequestHeader::read_from_prefix(req)
                .map_err(|_| error::IPC_ERROR_BAD_REQ_LEN)?;
            let payload_len = hdr.payload_length();
            let payload = rest.get(..payload_len).ok_or(error::IPC_ERROR_BAD_REQ_LEN)?;
            let out = data
                .get_mut(FlashResponseHeader::SIZE..)
                .ok_or(error::IPC_ERROR_BAD_REQ_LEN)?;

            match hdr.opcode {
                IPC_OP_FLASH_GET_INFO => {
                    if payload_len != 0 {
                        return Err(error::IPC_ERROR_BAD_REQ_LEN);
                    }
                    let result = self.handle_get_info(out)?;
                    rsp_hdr = FlashResponseHeader::success(result.len(), result.len() as u32);
                    rsp_len += result.len();
                }
                IPC_OP_FLASH_ERASE => {
                    if payload_len != core::mem::size_of::<FlashEraseRequest>() {
                        return Err(error::IPC_ERROR_BAD_REQ_LEN);
                    }
                    self.handle_erase(payload)?;
                    rsp_hdr = FlashResponseHeader::success(0, 0);
                }
                IPC_OP_FLASH_PROGRAM => {
                    let min_len = core::mem::size_of::<FlashProgramRequest>();
                    if payload_len < min_len {
                        return Err(error::IPC_ERROR_BAD_REQ_LEN);
                    }
                    self.handle_program(payload)?;
                    rsp_hdr = FlashResponseHeader::success(0, 0);
                }
                IPC_OP_FLASH_READ => {
                    if payload_len != core::mem::size_of::<FlashReadRequest>() {
                        return Err(error::IPC_ERROR_BAD_REQ_LEN);
                    }
                    let result = self.handle_read(payload, out)?;
                    rsp_hdr = FlashResponseHeader::success(result.len(), result.len() as u32);
                    rsp_len += result.len();
                }
                _ => return Err(error::IPC_ERROR_UNKNOWN_OP),
            }
            Ok(())
        })();

        if let Err(e) = outcome {
            rsp_hdr = FlashResponseHeader::from_error(e);
            rsp_len = FlashResponseHeader::SIZE;
        }

        data[..FlashResponseHeader::SIZE].copy_from_slice(rsp_hdr.as_bytes());
        ipc.respond(&data[..rsp_len])?;
        Ok(())
    }

    /// Runs the flash IPC server.
    ///
    /// This method enters an infinite loop, handling IPC requests one by one.
    pub fn run(&mut self, ipc: &IpcChannel, data: &mut [u8]) -> Result<(), ErrorCode> {
        loop {
            self.handle_one(ipc, data)?;
        }
    }
}

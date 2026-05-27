// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! Flash IPC wire format — opcodes, headers, and payload structs.
//!
//! This crate has no dependency on Pigweed, the kernel userspace, or any flash
//! HAL.  It can be compiled and tested on the host without a cross-compilation
//! toolchain.

#![no_std]

use util_types::Opcode;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Maximum data bytes carried by a single program/read payload.
pub const FLASH_IPC_MAX_DATA_LEN: usize = 2048;

/// IPC opcode for erasing a flash block.
pub const IPC_OP_FLASH_ERASE: Opcode = Opcode::new(*b"FLET");
/// IPC opcode for programming flash.
pub const IPC_OP_FLASH_PROGRAM: Opcode = Opcode::new(*b"FLWR");
/// IPC opcode for reading from flash.
pub const IPC_OP_FLASH_READ: Opcode = Opcode::new(*b"FLRD");
/// IPC opcode for retrieving flash information.
pub const IPC_OP_FLASH_GET_INFO: Opcode = Opcode::new(*b"FLIN");

/// Wire representation of a flash address.
///
/// Mirrors the layout of `hal_flash::FlashAddress` (`device_id`, `offset`) but
/// carries no dependency on the HAL crate.  Convert at the client/server
/// boundary using `From`/`Into` impls provided by `services_flash_opcode`.
#[derive(Default, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct WireAddress {
    pub device_id: u32,
    pub offset: u32,
}

impl WireAddress {
    pub const fn new(device_id: u32, offset: u32) -> Self {
        Self { device_id, offset }
    }
}

/// Common request header for all flash IPC operations.
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct FlashRequestHeader {
    pub opcode: Opcode,
    pub payload_len: u32,
    pub reserved: u32,
}

impl FlashRequestHeader {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn new(opcode: Opcode, payload_len: usize) -> Self {
        Self {
            opcode,
            payload_len: payload_len as u32,
            reserved: 0,
        }
    }

    pub fn payload_length(&self) -> usize {
        self.payload_len as usize
    }
}

/// Standard response header for all flash IPC operations.
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct FlashResponseHeader {
    pub status: u32,
    pub payload_len: u32,
    pub value: u32,
}

impl FlashResponseHeader {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn success(payload_len: usize, value: u32) -> Self {
        Self {
            status: 0,
            payload_len: payload_len as u32,
            value,
        }
    }

    /// Encode an error response.  `status` is the raw 32-bit error code;
    /// callers with access to `util_error::ErrorCode` should use
    /// `u32::from(err)`.
    pub fn error(status: u32) -> Self {
        Self {
            status,
            payload_len: 0,
            value: 0,
        }
    }

    pub fn payload_length(&self) -> usize {
        self.payload_len as usize
    }
}

/// Payload for `Erase` requests.
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct FlashEraseRequest {
    pub addr: WireAddress,
    pub size: u32,
}

/// Prefix payload for `Program` requests.
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct FlashProgramRequest {
    pub addr: WireAddress,
}

/// Payload for `Read` requests.
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct FlashReadRequest {
    pub addr: WireAddress,
    pub length: u32,
}

/// Information about the flash device returned by `GET_INFO`.
#[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C)]
pub struct FlashInfo {
    /// The size of a single flash page in bytes.
    pub page_size: u32,
    /// The total size of the flash in bytes.
    pub total_size: u32,
    /// A bitmap of supported erase block sizes.
    pub erasable_sizes_bitmap: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_header_roundtrip() {
        let hdr = FlashRequestHeader::new(IPC_OP_FLASH_READ, 64);
        let (parsed, rest) =
            FlashRequestHeader::read_from_prefix(hdr.as_bytes()).expect("header should decode");
        assert_eq!(rest.len(), 0);
        assert!(parsed.opcode == IPC_OP_FLASH_READ);
        assert_eq!(parsed.payload_length(), 64);
        assert_eq!(parsed.reserved, 0);
    }

    #[test]
    fn response_header_roundtrip_success() {
        let hdr = FlashResponseHeader::success(32, 0x1234_5678);
        let (parsed, rest) =
            FlashResponseHeader::read_from_prefix(hdr.as_bytes()).expect("header should decode");
        assert_eq!(rest.len(), 0);
        assert_eq!(parsed.status, 0);
        assert_eq!(parsed.payload_length(), 32);
        assert_eq!(parsed.value, 0x1234_5678);
    }

    #[test]
    fn response_header_error_encodes_status() {
        let raw_status: u32 = 0xDEAD_BEEF;
        let hdr = FlashResponseHeader::error(raw_status);
        assert_eq!(hdr.status, raw_status);
        assert_eq!(hdr.payload_length(), 0);
        assert_eq!(hdr.value, 0);
    }

    #[test]
    fn erase_payload_roundtrip() {
        let req = FlashEraseRequest {
            addr: WireAddress::new(2, 0x1000),
            size: 2048,
        };
        let (parsed, rest) =
            FlashEraseRequest::read_from_prefix(req.as_bytes()).expect("payload should decode");
        assert_eq!(rest.len(), 0);
        assert!(parsed.addr == WireAddress::new(2, 0x1000));
        assert_eq!(parsed.size, 2048);
    }

    #[test]
    fn program_payload_roundtrip() {
        let req = FlashProgramRequest {
            addr: WireAddress::new(1, 0x2000),
        };
        let (parsed, rest) =
            FlashProgramRequest::read_from_prefix(req.as_bytes()).expect("payload should decode");
        assert_eq!(rest.len(), 0);
        assert!(parsed.addr == WireAddress::new(1, 0x2000));
    }

    #[test]
    fn read_payload_roundtrip() {
        let req = FlashReadRequest {
            addr: WireAddress::new(3, 0x40),
            length: 512,
        };
        let (parsed, rest) =
            FlashReadRequest::read_from_prefix(req.as_bytes()).expect("payload should decode");
        assert_eq!(rest.len(), 0);
        assert!(parsed.addr == WireAddress::new(3, 0x40));
        assert_eq!(parsed.length, 512);
    }

    #[test]
    fn request_header_decode_fails_when_truncated() {
        let hdr = FlashRequestHeader::new(IPC_OP_FLASH_GET_INFO, 0);
        let truncated = &hdr.as_bytes()[..FlashRequestHeader::SIZE - 1];
        assert!(FlashRequestHeader::read_from_prefix(truncated).is_err());
    }

    #[test]
    fn response_header_decode_fails_when_truncated() {
        let hdr = FlashResponseHeader::success(0, 0);
        let truncated = &hdr.as_bytes()[..FlashResponseHeader::SIZE - 1];
        assert!(FlashResponseHeader::read_from_prefix(truncated).is_err());
    }
}

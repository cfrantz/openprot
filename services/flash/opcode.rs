// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! Shared flash IPC opcodes and data structures.
//!
//! This crate re-exports all wire types from `flash_wire` and adds the
//! HAL-aware conversion helpers that depend on `hal_flash_driver` and
//! `util_error`.

#![no_std]

pub use flash_wire::*;

use hal_flash_driver::FlashAddress;
use util_error::ErrorCode;

// ---------------------------------------------------------------------------
// FlashAddress <-> WireAddress conversions
// ---------------------------------------------------------------------------

impl From<FlashAddress> for WireAddress {
    fn from(a: FlashAddress) -> Self {
        WireAddress::new(a.device_id(), a.offset())
    }
}

impl From<WireAddress> for FlashAddress {
    fn from(w: WireAddress) -> Self {
        FlashAddress::new(w.device_id, w.offset)
    }
}

// ---------------------------------------------------------------------------
// Ergonomic error-response constructor that accepts ErrorCode
// ---------------------------------------------------------------------------

/// Extension trait that adds an `ErrorCode`-typed constructor to
/// [`FlashResponseHeader`].  Lives here so that `flash_wire` stays free of the
/// `util_error` / Pigweed dependency.
pub trait FlashResponseHeaderExt {
    fn from_error(err: ErrorCode) -> Self;
}

impl FlashResponseHeaderExt for FlashResponseHeader {
    fn from_error(err: ErrorCode) -> Self {
        FlashResponseHeader::error(u32::from(err))
    }
}

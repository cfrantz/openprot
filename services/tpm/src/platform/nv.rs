// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

use crate::platform::TpmPlatform;
use platform::types::{NvCompare, NvReadyState};
use platform::TpmNv;

impl TpmNv for TpmPlatform {
    fn enable(_param: *mut core::ffi::c_void, _size: usize) -> i32 {
        Self::with_state(|state| {
            state.nv_enabled = true;
            0 // Success
        })
    }

    fn disable(_param: *mut core::ffi::c_void, _size: usize) {
        Self::with_state(|state| {
            state.nv_enabled = false;
        });
    }

    fn read(offset: u32, size: u32, data: &mut [u8]) -> bool {
        Self::with_state(|state| {
            let offset = offset as usize;
            let size = size as usize;
            if offset + size > state.nv_ram.len() {
                return false;
            }
            data.copy_from_slice(&state.nv_ram[offset..offset + size]);
            true
        })
    }

    fn write(offset: u32, size: u32, data: &[u8]) -> bool {
        Self::with_state(|state| {
            let offset = offset as usize;
            let size = size as usize;
            if offset + size > state.nv_ram.len() {
                return false;
            }
            state.nv_ram[offset..offset + size].copy_from_slice(data);
            // Once we write to NV, we've "manufactured" the TPM state.
            state.manufacture_needed = false;
            true
        })
    }

    fn commit() -> i32 {
        0 // No-op: RAM storage is always "committed"
    }

    fn clear(offset: u32, size: u32) -> i32 {
        Self::with_state(|state| {
            let offset = offset as usize;
            let size = size as usize;
            if offset + size > state.nv_ram.len() {
                return -1;
            }
            state.nv_ram[offset..offset + size].fill(0xff);
            0
        })
    }

    fn move_block(src_offset: u32, dest_offset: u32, size: u32) -> i32 {
        Self::with_state(|state| {
            let src = src_offset as usize;
            let dest = dest_offset as usize;
            let size = size as usize;
            if src + size > state.nv_ram.len() || dest + size > state.nv_ram.len() {
                return -1;
            }
            state.nv_ram.copy_within(src..src + size, dest);
            0
        })
    }

    fn get_ready_state() -> NvReadyState {
        NvReadyState::Ready
    }

    fn get_changed_status(offset: u32, size: u32, data: &[u8]) -> NvCompare {
        Self::with_state(|state| {
            let offset = offset as usize;
            let size = size as usize;
            if offset + size > state.nv_ram.len() {
                return NvCompare::Invalid;
            }
            if &state.nv_ram[offset..offset + size] == data {
                NvCompare::Same
            } else {
                NvCompare::Different
            }
        })
    }

    fn needs_manufacture() -> bool {
        Self::with_state(|state| state.manufacture_needed)
    }
}

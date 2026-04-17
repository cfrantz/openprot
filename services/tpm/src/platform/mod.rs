// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use core::cell::RefCell;
use platform::types::SpecCapabilityValue;
use platform::{
    implement_tpm_cancel, implement_tpm_clock, implement_tpm_control, implement_tpm_entropy,
    implement_tpm_fail, implement_tpm_info, implement_tpm_lifecycle, implement_tpm_locality,
    implement_tpm_nv, implement_tpm_pcr, implement_tpm_secrets, implement_tpm_virtual_nv,
};
use platform::{
    TpmCancel, TpmControl, TpmEntropy, TpmFail, TpmInfo, TpmLocality, TpmPcr, TpmSecrets,
    TpmVirtualNv,
};

mod clock;
mod lifecycle;
mod nv;

pub struct PlatformState {
    pub nv_ram: [u8; 8192],
    pub power_lost: bool,
    pub timer_reset: bool,
    pub timer_stopped: bool,
    pub nv_enabled: bool,
    pub manufacture_needed: bool,
    pub locality: u8,
}

impl PlatformState {
    pub const fn new() -> Self {
        Self {
            nv_ram: [0xffu8; 8192],
            power_lost: true,
            timer_reset: true,
            timer_stopped: true,
            nv_enabled: false,
            manufacture_needed: true,
            locality: 0,
        }
    }
}

pub struct TpmPlatform;

struct GlobalState<T>(RefCell<T>);
unsafe impl<T> Sync for GlobalState<T> {}

static STATE: GlobalState<PlatformState> = GlobalState(RefCell::new(PlatformState::new()));

impl TpmPlatform {
    fn with_state<R, F: FnOnce(&mut PlatformState) -> R>(f: F) -> R {
        let mut state = STATE.0.borrow_mut();
        f(&mut state)
    }
}

impl TpmEntropy for TpmPlatform {
    fn get_entropy(entropy: &mut [u8]) -> i32 {
        for b in entropy.iter_mut() {
            *b = 0x42; // Not very random, but works for dummy.
        }
        entropy.len() as i32
    }
}

impl TpmLocality for TpmPlatform {
    fn get() -> u8 {
        Self::with_state(|s| s.locality)
    }
    fn set(locality: u8) {
        Self::with_state(|s| s.locality = locality);
    }
}

impl TpmCancel for TpmPlatform {
    fn is_canceled() -> bool {
        false
    }
    fn set() {}
    fn clear() {}
}

impl TpmPcr for TpmPlatform {
    fn number_of_pcrs() -> u32 {
        24
    }
    fn get_attributes(_pcr: u32) -> u32 {
        0
    }
    fn get_initial_value(_pcr: u32, _alg: u16, _locality: u8, buffer: &mut [u8]) -> u16 {
        for b in buffer.iter_mut() {
            *b = 0;
        }
        0
    }
    fn is_bank_default_active(_alg: u16) -> bool {
        true
    }
}

impl TpmInfo for TpmPlatform {
    fn get_manufacturer_code() -> u32 {
        0x474f4f47
    } // "GOOG"
    fn get_vendor_code(_index: i32) -> u32 {
        0
    }
    fn get_vendor_type() -> u32 {
        0
    }
    fn get_firmware_version_high() -> u32 {
        1
    }
    fn get_firmware_version_low() -> u32 {
        0
    }
    fn get_firmware_svn() -> u16 {
        1
    }
    fn get_firmware_max_svn() -> u16 {
        1
    }
    fn get_spec_capability(data: &mut SpecCapabilityValue) {
        *data = SpecCapabilityValue {
            tpm_spec_level: 0,
            tpm_spec_version: 200,
            tpm_spec_year: 2018,
            tpm_spec_day_of_year: 1,
            platform_family: 0,
            platform_level: 0,
            platform_revision: 0,
            platform_year: 0,
            platform_day_of_year: 0,
        };
    }
    fn get_manufacture_data(_data: &mut [u8]) {}
    fn get_enabled_self_test(_full_test: u8, _to_test_vector: &mut [u8]) {}
}

impl TpmVirtualNv for TpmPlatform {
    fn is_virtual_index(_handle: u32) -> bool {
        false
    }
    fn read(_handle: u32, _offset: u32, _size: u32, _buffer: &mut [u8]) -> i32 {
        -1
    }
    fn read_public(_handle: u32, _buffer: &mut [u8]) -> u16 {
        0x0080
    } // TPM_RC_HANDLE
    fn populate_info(_handle: u32, _info: *mut core::ffi::c_void) {}
    fn cap_get_index(_handle: u32) -> u32 {
        0
    }
    fn operation_accepts_virtual_handles(_handle: u32) -> bool {
        false
    }
}

impl TpmFail for TpmPlatform {
    fn fail(_function: Option<&str>, _line: i32, _location: u64, _code: i32) {}
    fn in_failure_mode() -> bool {
        false
    }
    fn get_code() -> u32 {
        0
    }
    fn get_location() -> u64 {
        0
    }
    fn get_function_name() -> Option<&'static str> {
        None
    }
    fn get_line() -> u32 {
        0
    }
}

impl TpmSecrets for TpmPlatform {
    fn get_firmware_secret(_buffer: &mut [u8]) -> i32 {
        -1
    }
    fn get_firmware_svn_secret(_svn: u16, _buffer: &mut [u8]) -> i32 {
        -1
    }
}

impl TpmControl for TpmPlatform {
    fn set_force_failure_mode() {}
    fn set_nv_avail(_avail: bool) {}
    fn set_tpm_firmware_hash(_hash: u32) {}
    fn set_tpm_firmware_svn(_svn: u16) {}
    fn set_physical_presence(_on: bool) {}
    fn physical_presence_asserted() -> bool {
        true
    }
}

implement_tpm_clock!(TpmPlatform);
implement_tpm_nv!(TpmPlatform);
implement_tpm_lifecycle!(TpmPlatform);
implement_tpm_entropy!(TpmPlatform);
implement_tpm_locality!(TpmPlatform);
implement_tpm_cancel!(TpmPlatform);
implement_tpm_pcr!(TpmPlatform);
implement_tpm_info!(TpmPlatform);
implement_tpm_virtual_nv!(TpmPlatform);
implement_tpm_fail!(TpmPlatform);
implement_tpm_secrets!(TpmPlatform);
implement_tpm_control!(TpmPlatform);

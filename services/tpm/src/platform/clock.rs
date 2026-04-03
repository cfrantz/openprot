// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

use crate::platform::TpmPlatform;
use platform::types::{ClockAdjust, ClockDirection};
use platform::TpmClock;

#[cfg(feature = "silicon")]
const TICK_HZ: u64 = 100_000_000;
#[cfg(feature = "fpga")]
const TICK_HZ: u64 = 6_000_000;
#[cfg(feature = "verilator")]
const TICK_HZ: u64 = 125_000;

// The RISC-V MTIME register is a 64-bit counter.
// On Earlgrey, it is at TIMER_BASE + 0x110.
const MTIME_REGISTER: *const u64 = (0x4010_0000 + 0x110) as *const u64;

fn read_mtime() -> u64 {
    // In an experimental environment, we might not have direct access to the
    // register. For now, we'll assume it's mapped or accessible.
    unsafe { core::ptr::read_volatile(MTIME_REGISTER) }
}

fn ticks_to_ms(ticks: u64) -> u64 {
    ticks.saturating_mul(1000) / TICK_HZ
}

impl TpmClock for TpmPlatform {
    fn read() -> u64 {
        ticks_to_ms(read_mtime())
    }

    fn real_time() -> u64 {
        // For a root-of-trust, wall clock time might not be available.
        // Returning the monotonic time is a common fallback.
        Self::read()
    }

    fn reset() {
        Self::with_state(|state| {
            state.timer_reset = true;
        });
    }

    fn restart() {
        Self::with_state(|state| {
            state.timer_stopped = true;
        });
    }

    fn was_reset() -> bool {
        Self::with_state(|state| {
            let reset = state.timer_reset;
            state.timer_reset = false;
            reset
        })
    }

    fn was_stopped() -> bool {
        Self::with_state(|state| {
            let stopped = state.timer_stopped;
            state.timer_stopped = false;
            stopped
        })
    }

    fn adjust_rate(_step: ClockAdjust, _direction: ClockDirection) {
        // Stub: Rate adjustment is not implemented in this experimental version.
    }
}

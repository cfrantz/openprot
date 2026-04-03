// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

use crate::platform::TpmPlatform;
use platform::TpmLifecycle;

impl TpmLifecycle for TpmPlatform {
    fn power_on() -> i32 {
        Self::with_state(|state| {
            state.power_lost = true;
            state.timer_reset = true;
            0 // Success
        })
    }

    fn reset() -> i32 {
        Self::with_state(|state| {
            state.timer_reset = true;
            0 // Success
        })
    }

    fn power_off() {}

    fn tear_down() {}

    fn was_power_lost() -> bool {
        Self::with_state(|state| {
            let lost = state.power_lost;
            state.power_lost = false;
            lost
        })
    }

    fn init_start() {}

    fn init_end_ok() {}
}

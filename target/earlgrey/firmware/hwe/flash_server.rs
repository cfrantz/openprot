// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]
use pw_status::Result;
use userspace::time::{sleep_until, Instant};
use userspace::{entry, syscall};

/*
 * TODO: implement flash server.
 */

#[entry]
fn entry() -> Result<()> {
    loop {
        let wake_time = syscall::debug_clock_now().ticks() + 10_000_000;
        sleep_until(Instant::from_ticks(wake_time))?;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    pw_log::error!("FAIL: panic in {}", module_path!() as &str);
    loop {}
}

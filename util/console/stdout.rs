// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

use std::io::Write;

// This is a low-level console output function that works with host-based
// code.  This simply outputs to stdout.
#[unsafe(no_mangle)]
pub extern "C" fn system_lowlevel_console_write(ptr: *const u8, length: usize) {
    let bytes = unsafe { core::slice::from_raw_parts(ptr, length) };
    let _ = std::io::stdout().write_all(bytes);
    let _ = std::io::stdout().flush();
}

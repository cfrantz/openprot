// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! Console
//!
//! This crate provides basic console functionality.

#![no_std]

use core::convert::Infallible;

pub use ufmt;
use ufmt::uWrite;
pub use ufmt::{uwrite, uwriteln};

pub struct Console;

//#[cfg(target_os = "none")]
//pub use console_pigweed::system_lowlevel_console_write;
//#[cfg(not(target_os = "none"))]
//pub use console_stdout::system_lowlevel_console_write;

//#[cfg(target_os = "none")]
//pub extern "Rust" fn pigweed_debug_log(bytes: &[u8]) {
//    use userspace::syscall;
//    let _ = syscall::debug_log(bytes);
//}

unsafe extern "C" {
    fn system_lowlevel_console_write(ptr: *const u8, length: usize);
}

impl uWrite for Console {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
        unsafe { system_lowlevel_console_write(s.as_ptr(), s.len()) };
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::ufmt;
        $crate::uwrite!(&mut $crate::Console, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use $crate::ufmt;
        $crate::ufmt::uwriteln!(&mut $crate::Console, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(feature = "trace") {
            use $crate::ufmt;
            $crate::uwrite!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
}

#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {
        if cfg!(feature = "trace") {
            use $crate::ufmt;
            $crate::uwriteln!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
}

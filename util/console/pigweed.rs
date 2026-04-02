// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

#![no_std]
use core::arch::naked_asm;

//use userspace::syscall;

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn system_lowlevel_console_write(ptr: *const u8, length: usize) {
    naked_asm!("
            li t0, {id}
            ecall
            ret
            ",
        id = const 0xf002 as u32,
    );
    //let _ = syscall::debug_log(bytes);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

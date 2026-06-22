// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use pw_status::Error;
use userspace::{process_entry, syscall};
use util_error::{AsStatus, ErrorCode};
use util_zfmt::messages::{ProcessExit, ProcessStart};

fn platform_server() -> Result<(), ErrorCode> {
    use earlgrey_platform::server::PlatformServer;
    use platform_codegen::{handle, signals};
    use userspace::syscall::Signals;
    use userspace::time::{Clock, Duration, SystemClock};
    use util_ipc::IpcHandle;

    let mut server = PlatformServer::new(IpcHandle::new(handle::SYSMGR_PLATFORM))?;
    server.set_exit_deadline(SystemClock::now() + Duration::from_secs(10));
    server.start()?;

    let usb_sig = signals::GPIO_16;
    let rst0_sig = signals::GPIO_17;
    let rst1_sig = signals::GPIO_18;

    loop {
        if server.should_exit() {
            return Ok(());
        }
        let deadline = server.next_deadline();
        let wait_res =
            syscall::object_wait(handle::PLATFORM_INTERRUPTS, Signals::READABLE, deadline);

        match wait_res {
            Ok(wait_return) => {
                let signals = wait_return.pending_signals;

                if (signals & usb_sig) != Signals::empty() {
                    server.handle_usb_presence_interrupt()?;
                }
                if (signals & rst0_sig) != Signals::empty() {
                    server.handle_rst_mon_interrupt(0)?;
                }
                if (signals & rst1_sig) != Signals::empty() {
                    server.handle_rst_mon_interrupt(1)?;
                }

                syscall::interrupt_ack(handle::PLATFORM_INTERRUPTS, signals)
                    .map_err(ErrorCode::kernel_error)?;
            }
            Err(Error::DeadlineExceeded) => {
                if server.should_exit() {
                    return Ok(());
                }
                server.handle_timeout()?;
            }
            Err(e) => {
                return Err(ErrorCode::kernel_error(e));
            }
        }
    }
}

#[process_entry("platform")]
fn entry() -> Result<(), Error> {
    util_zfmt::info!(ProcessStart { name: "platform" });
    let ret = platform_server();
    util_zfmt::error!(ProcessExit {
        name: "platform",
        status: ret.as_status()
    });

    let status_res = ret.map_err(|_| Error::Unknown);
    syscall::debug_shutdown(status_res)
}

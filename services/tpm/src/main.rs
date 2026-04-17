// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use app_tpm_service::handle;
use pw_status::Result;
use userspace::syscall::Signals;
use userspace::time::Instant;
use userspace::{entry, syscall};
use platform::TpmNv;

use crypto_client::backend::CryptoClient;

use util_misc::hexdump;

// Use the platform implementation to ensure its symbols are linked.
use tpm_platform as _;
use tpm_platform::tpm_cc::TpmCC;

unsafe extern "C" {
    fn ExecuteCommand(reqsize: u32, req: *const u8, rspsize: *mut u32, rsp: *mut *mut u8);
    fn _TPM_Init();
    fn TPM_Manufacture(firstTime: i32) -> i32;
}

fn tpm_init() {
    pw_log::info!("TPM: Initializing platform...");
    tpm_platform::tpm_crypto::NullCrypto::initialize(CryptoClient::new(handle::CRYPTOLIB));
    pw_log::info!("TPM: IPC handle set to {}", handle::CRYPTOLIB);

    // These calls are normally made by the platform wrapper, but we'll 
    // ensure the state is set up correctly here.
    // TpmPlatform implements TpmLifecycle, which our implement_tpm_lifecycle macro
    // exports as _plat__Signal_PowerOn, etc.
    // In our case, _TPM_Init will call _plat__WasPowerLost.
    
    unsafe {
        _TPM_Init();
    }

    if true || tpm_platform::platform::TpmPlatform::needs_manufacture() {
        pw_log::info!("TPM: Manufacturing...");
        unsafe {
            TPM_Manufacture(1);
        }
    }
    pw_log::info!("TPM: Initialization complete.");
}

fn handle_ipc() -> Result<()> {
    // 4KB buffer for TPM commands/responses.
    let mut cmd_buf = [0u8; 4096];
    let mut resp_buf = [0u8; 4096];

    loop {
        // Wait for an IPC request.
        let wait_return = syscall::object_wait(handle::IPC, Signals::READABLE, Instant::MAX)?;

        if !wait_return.pending_signals.contains(Signals::READABLE) {
            continue;
        }

        pw_log::info!("TPM wakeup");
        // Read the command from the channel.
        let cmd_len = syscall::channel_read(handle::IPC, 0, &mut cmd_buf)?;
        if cmd_len == 0 {
            continue;
        }
        if let Some(bytes) = cmd_buf.get(6..10) {
            let val = TpmCC(u32::from_be_bytes(bytes.try_into().unwrap()));
            pw_log::info!("TPM command: {}", val.as_str() as &str);
        }

        hexdump(&cmd_buf[..cmd_len]);

        let mut rspsize = resp_buf.len() as u32;
        let mut rspptr = resp_buf.as_mut_ptr();

        pw_log::info!("TPM dispatch: rsp={:x} len={}", rspptr as usize, rspsize as usize);
        // Execute the TPM command.
        unsafe {
            ExecuteCommand(
                cmd_len as u32,
                cmd_buf.as_ptr(),
                &mut rspsize,
                &mut rspptr,
            );
        }
        pw_log::info!("TPM returns: rsp={:x} len={}", rspptr as usize, rspsize as usize);

        // If the C code returned a different pointer, we must copy it back.
        // (Though with our current setup, it should stay in resp_buf).
        if rspptr != resp_buf.as_mut_ptr() {
            let external_rsp = unsafe { core::slice::from_raw_parts(rspptr, rspsize as usize) };
            let copy_len = core::cmp::min(rspsize as usize, resp_buf.len());
            resp_buf[..copy_len].copy_from_slice(&external_rsp[..copy_len]);
            syscall::channel_respond(handle::IPC, &resp_buf[..copy_len])?;
        } else {
            syscall::channel_respond(handle::IPC, &resp_buf[..rspsize as usize])?;
        }
    }
}

#[entry]
fn entry() -> ! {
    pw_log::info!("TPM: Service starting...");
    
    tpm_init();

    let ret = handle_ipc();
    if let Err(e) = ret {
        pw_log::error!("TPM: IPC handler failed: {}", e as u32);
    }

    let _ = syscall::debug_shutdown(ret);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    pw_log::error!("TPM: PANIC!");
    let _ = syscall::debug_shutdown(Err(pw_status::Error::Unknown));
    loop {}
}

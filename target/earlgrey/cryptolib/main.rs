#![no_main]
#![no_std]

//use app_cryptolib::handle;
use pw_status::{Result, StatusCode};
use userspace::entry;
//use ipc::crypto::*;
use app_cryptolib::handle;
use crypto_server::Server;
use userspace::syscall::{self, Signals};
use userspace::time::Instant;
use zerocopy::FromBytes;

#[unsafe(no_mangle)]
pub static kDeviceType: u32 = 2;

fn run(ipc: u32) -> Result<()> {
    let mut server = Server::default();
    let mut req = [0u8; 2080];
    let mut rsp = [0u8; 2080];
    loop {
        // Wait for a Request to come in.
        //pw_log::debug!("crypto: Waiting for request");
        syscall::object_wait(ipc, Signals::READABLE, Instant::MAX)?;

        let rqlen = syscall::channel_read(ipc, 0, &mut req)?;
        let rslen = {
            let (status, rsp) = u32::mut_from_prefix(&mut rsp).unwrap();
            match server.exec(&mut req[..rqlen], rsp) {
                Ok(r) => {
                    *status = 0;
                    4 + r.len()
                }
                Err(e) => {
                    *status = e as u32;
                    4
                }
            }
        };
        syscall::channel_respond(ipc, &rsp[..rslen])?;
    }
}

#[entry]
fn entry() -> ! {
    pw_log::info!("Starting cryptolib service");
    let ret = run(handle::CRYPTOLIB);

    pw_log::info!("Cryptolib ended with {}", ret.status_code() as u32);
    // Since this is written as a test, shut down with the return status from `main()`.
    let _ = syscall::debug_shutdown(ret);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    pw_log::info!("Cryptolib panic");
    loop {}
}

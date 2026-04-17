use crypto::{implement_tpm_rand, rand::TpmRand};
use crypto_traits::digest::{Digest, DigestFinal, DigestInit, DigestUpdate, Sha2_256};
use tpm_types::*;
//use crypto_client::sha2::{Sha2Context};
use crypto_client::backend::CryptoClient;

use crate::tpm_crypto::NullCrypto;
use pw_status::Error;
use rv_core_ibex::RvCoreIbex;
use zerocopy::IntoBytes;

unsafe extern "C" {
    fn CryptKDFa(
        hash_alg: TpmAlgId,       // IN: hash algorithm used in HMAC
        key: *const Tpm2B,        // IN: HMAC key
        label: *const Tpm2B,      // IN: a label for the KDF
        context_u: *const Tpm2B,  // IN: context U
        context_v: *const Tpm2B,  // IN: context V
        size_in_bits: u32,        // IN: size of generated key in bits
        key_stream: *mut u8,      // OUT: key buffer
        counter_in_out: *mut u32, // IN/OUT: caller may provide the iteration
        //     counter for incremental operations to avoid large intermediate buffers.
        blocks: u16, // IN: If non-zero, this is the maximum number
                     //     of blocks to be returned, regardless of sizeInBits
    ) -> u16;

}

#[repr(C)]
struct FakeDrbg {
    counter: u64,
    magic: u32,
    seed: [u32; 8],
}

impl FakeDrbg {
    const fn new() -> Self {
        Self {
            counter: 0,
            magic: DrbgState::MAGIC,
            seed: [0u32; 8],
        }
    }

    fn init(
        &mut self,
        client: &CryptoClient,
        seed: &[u8],
        purpose: &[u8],
        name: &[u8],
        additional: &[u8],
    ) -> Result<(), Error> {
        let p = self as *const Self;
        pw_log::info!("fakedrbg: init {:x}", p as usize);
        let handle = client.init(&Sha2_256)?;
        client.update(&handle, seed)?;
        client.update(&handle, purpose)?;
        client.update(&handle, name)?;
        client.update(&handle, additional)?;
        let digest = client.finalize(handle)?;
        self.counter = 0;
        self.magic = DrbgState::MAGIC;
        self.seed.as_mut_bytes().copy_from_slice(digest.digest());
        pw_log::info!("fakedrbg: initok");
        Ok(())
    }

    fn additional_data(&mut self, client: &CryptoClient, data: &[u8]) -> Result<(), Error> {
        pw_log::info!("fakedrbg: add");
        let handle = client.init(&Sha2_256)?;
        client.update(&handle, data)?;
        let digest = client.finalize(handle)?;
        self.seed.as_mut_bytes().copy_from_slice(digest.digest());
        pw_log::info!("fakedrbg: addok");
        Ok(())
    }

    fn fill_bytes(&mut self, client: &CryptoClient, data: &mut [u8]) -> Result<(), Error> {
        pw_log::info!("fakedrbg: fill bytes {}", data.len() as usize);
        for d in data.chunks_mut(32) {
            let handle = client.init(&Sha2_256)?;
            self.counter += 1;
            client.update(&handle, &self.counter.as_bytes())?;
            client.update(&handle, &self.seed.as_bytes())?;
            let digest = client.finalize(handle)?;
            d.copy_from_slice(&digest.digest()[..d.len()]);
        }
        pw_log::info!("fakedrbg: fillok");
        Ok(())
    }

    fn uninit(&mut self) {
        self.counter = 0;
        self.magic = DrbgState::INVALID_MAGIC;
        self.seed.fill(0);
    }

    fn as_rand_state(&mut self) -> &mut RandState {
        unsafe { core::mem::transmute(self) }
    }
}

static mut BASE_DRBG: FakeDrbg = FakeDrbg::new();

impl TpmRand for NullCrypto {
    fn rand_subsystem_init(&self) -> bool {
        true
    }
    fn rand_subsystem_startup(&self) -> bool {
        true
    }
    fn rand_generate(&self, buffer: &mut [u8]) -> u16 {
        let ibex = unsafe { RvCoreIbex::new() };
        let regs = ibex.regs();

        for chunk in buffer.chunks_mut(4) {
            while !regs.rnd_status().read().rnd_data_valid() {}
            let data = u32::from(regs.rnd_data().read());
            let len = chunk.len();
            chunk.copy_from_slice(&data.to_le_bytes()[..len]);
        }
        buffer.len() as u16
    }
    fn rand_stir(&self) -> TpmRc {
        TpmRc::Success
    }

    fn rand_drbg_generate(&self, state: Option<&mut RandState>, buffer: &mut [u8]) -> u16 {
        if buffer.is_empty() {
            return 0;
        }

        #[allow(static_mut_refs)]
        let state = state.unwrap_or_else(|| {
            pw_log::info!("Using BASE_DRBG");
            unsafe { BASE_DRBG.as_rand_state() }
        });
        let p = state as *const RandState;
        pw_log::info!("drbg_generate {:x}", p as usize);
        let magic = unsafe { state.drbg.magic };
        match magic {
            DrbgState::MAGIC => unsafe {
                pw_log::info!("Rand: drbg");
                // Translate state to handle.
                let state = core::mem::transmute::<&mut RandState, &mut FakeDrbg>(state);
                match state.fill_bytes(&self.client, buffer) {
                    Ok(_) => buffer.len() as u16,
                    Err(e) => {
                        pw_log::error!("drbg_generate error: {}", e as u32);
                        0
                    }
                }
            },
            KdfState::MAGIC => unsafe {
                pw_log::info!("Rand: KDFa");
                let state = core::mem::transmute::<&mut RandState, &mut KdfState>(state);
                let mut counter = state.counter as u32;
                let rv = CryptKDFa(
                    state.hash,
                    state.seed,
                    state.label,
                    state.context,
                    core::ptr::null(),
                    state.limit.min(buffer.len() as u32 * 8),
                    buffer.as_mut_ptr(),
                    &mut counter,
                    0,
                );
                state.counter = counter as u64;
                rv
            },
            _ => {
                pw_log::error!("Bad DRBG magic={:08x}", magic as u32);
                0
            }
        }
    }

    fn rand_drbg_additional_data(&self, state: &mut DrbgState, data: &[u8]) {
        let state = unsafe { core::mem::transmute::<&mut DrbgState, &mut FakeDrbg>(state) };
        match state.additional_data(&self.client, data) {
            Ok(()) => {}
            Err(e) => {
                pw_log::error!("drbg_additional_data error: {}", e as u32);
            }
        }
    }

    fn rand_drbg_instantiate_seeded(
        &self,
        state: &mut DrbgState,
        seed: &[u8],
        purpose: &[u8],
        name: &[u8],
        additional: &[u8],
    ) -> TpmRc {
        let state = unsafe { core::mem::transmute::<&mut DrbgState, &mut FakeDrbg>(state) };
        match state.init(&self.client, seed, purpose, name, additional) {
            Ok(()) => TpmRc::Success,
            Err(e) => {
                pw_log::error!("drbg_instantiate_seeded error: {}", e as u32);
                TpmRc::Failure
            }
        }
    }

    fn rand_drbg_uninstantiate(&self, state: &mut DrbgState) -> TpmRc {
        if state.magic == DrbgState::MAGIC {
            unsafe {
                // Translate state to handle.
                let state = core::mem::transmute::<&mut DrbgState, &mut FakeDrbg>(state);
                state.uninit();
            };
            TpmRc::Success
        } else {
            TpmRc::Value
        }
    }
}

implement_tpm_rand!(NullCrypto);

use crypto::{
    implement_tpm_rand,
    rand::TpmRand,
};
use tpm_types::*;

use crate::tpm_crypto::NullCrypto;
use rv_core_ibex::RvCoreIbex;

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
    fn rand_drbg_generate(&self, _state: Option<&mut RandState>, buffer: &mut [u8]) -> u16 {
        buffer.fill(0);
        buffer.len() as u16
    }
    fn rand_drbg_additional_data(&self, _state: &mut DrbgState, _data: &[u8]) {}
    fn rand_drbg_instantiate_seeded(
        &self,
        _state: &mut DrbgState,
        _seed: &[u8],
        _purpose: &[u8],
        _name: &[u8],
        _additional: &[u8],
    ) -> TpmRc {
        TpmRc::Success
    }
    fn rand_drbg_uninstantiate(&self, _state: &mut DrbgState) -> TpmRc {
        TpmRc::Success
    }
}

implement_tpm_rand!(NullCrypto);

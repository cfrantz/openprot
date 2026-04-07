#![allow(unused_imports)]
use crypto_common::Opcode;
use otcrypto::{CryptoInterface, HashDigest, HashMode, OtCrypto, Sha2Context};
use pw_status::{Error, Result};
use zerocopy::{FromBytes, FromZeros};

pub struct Sha2Contexts<const N: usize> {
    context: [Option<Sha2Context>; N],
}

impl<const N: usize> Default for Sha2Contexts<N> {
    fn default() -> Self {
        Sha2Contexts {
            context: [const { None }; N],
        }
    }
}

impl<const N: usize> Sha2Contexts<N> {
    fn alloc(&mut self, context: Sha2Context) -> Result<u32> {
        for (i, ctx) in self.context.iter_mut().enumerate() {
            if ctx.is_none() {
                ctx.replace(context);
                return Ok(i as u32);
            }
        }
        Err(Error::ResourceExhausted)
    }

    pub fn init<'a>(&mut self, op: Opcode, _req: &[u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
        let hash_mode = match op {
            Opcode::SHA2_256_INIT => HashMode::Sha256,
            Opcode::SHA2_384_INIT => HashMode::Sha384,
            Opcode::SHA2_512_INIT => HashMode::Sha512,
            _ => return Err(Error::InvalidArgument),
        };

        let len = {
            let mut context = Sha2Context::new_zeroed();
            let (index, _) = u32::mut_from_prefix(rsp).map_err(|_| Error::Internal)?;
            OtCrypto::sha2_init(hash_mode, &mut context)?;
            *index = self.alloc(context)?;
            core::mem::size_of::<u32>()
        };
        Ok(&rsp[..len])
    }

    pub fn update<'a>(
        &mut self,
        _op: Opcode,
        req: &mut [u8],
        rsp: &'a mut [u8],
    ) -> Result<&'a [u8]> {
        let (&index, data) = u32::ref_from_prefix(req).map_err(|_| Error::Internal)?;
        let index = index as usize;
        if index >= N {
            return Err(Error::OutOfRange);
        }
        if let Some(context) = self.context[index as usize].as_mut() {
            OtCrypto::sha2_update(context, data)?;
            Ok(&rsp[0..0])
        } else {
            Err(Error::InvalidArgument)
        }
    }

    pub fn finalize<'a>(
        &mut self,
        op: Opcode,
        req: &mut [u8],
        rsp: &'a mut [u8],
    ) -> Result<&'a [u8]> {
        let hash_words = match op {
            Opcode::SHA2_256_FINAL => 256 / 32,
            Opcode::SHA2_384_FINAL => 384 / 32,
            Opcode::SHA2_512_FINAL => 512 / 32,
            _ => return Err(Error::InvalidArgument),
        };
        let (&index, _rest) = u32::ref_from_prefix(req).map_err(|_| Error::Internal)?;
        let index = index as usize;
        if index >= N {
            return Err(Error::OutOfRange);
        }
        let len = if let Some(context) = self.context[index as usize].as_mut() {
            let (digest, _rest) = HashDigest::mut_from_prefix_with_elems(rsp, hash_words)
                .map_err(|_| Error::Internal)?;
            OtCrypto::sha2_final(context, digest)?;
            core::mem::size_of_val(digest)
        } else {
            return Err(Error::InvalidArgument);
        };
        let _ = self.context[index as usize].take();
        Ok(&rsp[..len])
    }
}

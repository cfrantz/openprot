use crypto_common::Opcode;
use otcrypto::{BlindedKey, CryptoInterface, HmacContext, OtCrypto};
use pw_status::{Error, Result};
use zerocopy::{FromBytes, FromZeros};

pub struct HmacContexts<const N: usize> {
    context: [Option<HmacContext>; N],
}

impl<const N: usize> Default for HmacContexts<N> {
    fn default() -> Self {
        HmacContexts {
            context: [const { None }; N],
        }
    }
}

impl<const N: usize> HmacContexts<N> {
    fn alloc(&mut self, context: HmacContext) -> Result<u32> {
        for (i, ctx) in self.context.iter_mut().enumerate() {
            if ctx.is_none() {
                ctx.replace(context);
                return Ok(i as u32);
            }
        }
        Err(Error::ResourceExhausted)
    }

    pub fn init<'a>(&mut self, _op: Opcode, req: &mut [u8], rsp: &'a mut [u8]) -> Result<&'a [u8]> {
        // HMAC key size and hash mode are determined by the opcode or the key itself in otcrypto.
        // For now, assume req contains the raw key material.
        // Note: otcrypto's hmac_init takes a BlindedKey.
        // We'll need a way to construct a BlindedKey from raw bytes if the client sends raw bytes.

        // TODO: This assumes the client sends a BlindedKey structure.
        let (key, material) = BlindedKey::mut_from_prefix(req).map_err(|_| Error::Internal)?;

        let len = {
            let mut context = HmacContext::new_zeroed();
            let (index, _) = u32::mut_from_prefix(rsp).map_err(|_| Error::Internal)?;
            OtCrypto::hmac_init(&mut context, key.with_key_material(material))?;
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
        if let Some(context) = self.context[index].as_mut() {
            OtCrypto::hmac_update(context, data)?;
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
        let tag_words = match op {
            Opcode::HMAC_SHA2_256_FINAL => 256 / 32,
            Opcode::HMAC_SHA2_384_FINAL => 384 / 32,
            Opcode::HMAC_SHA2_512_FINAL => 512 / 32,
            _ => return Err(Error::InvalidArgument),
        };
        let (&index, _rest) = u32::ref_from_prefix(req).map_err(|_| Error::Internal)?;
        let index = index as usize;
        if index >= N {
            return Err(Error::OutOfRange);
        }
        let len = if let Some(context) = self.context[index].as_mut() {
            let tag_bytes = &mut rsp[..tag_words * 4];
            OtCrypto::hmac_final(context, tag_bytes)?;
            tag_words * 4
        } else {
            return Err(Error::InvalidArgument);
        };
        let _ = self.context[index].take();
        Ok(&rsp[..len])
    }
}

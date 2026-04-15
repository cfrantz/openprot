#![no_std]
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

pub mod keytypes;
mod opcode;
pub mod symmetric_key;

pub use opcode::Opcode;

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct CipherMode(u32);
impl CipherMode {
    pub const ECB: Self = Self(u32::from_le_bytes(*b"ECB_"));
    pub const CBC: Self = Self(u32::from_le_bytes(*b"CBC_"));
    pub const CFB: Self = Self(u32::from_le_bytes(*b"CFB_"));
    pub const CTR: Self = Self(u32::from_le_bytes(*b"CTR_"));
    pub const OFB: Self = Self(u32::from_le_bytes(*b"OFB_"));
}

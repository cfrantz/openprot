#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct Opcode(u16, u16);

impl Opcode {
    const fn fourcc(code: [u8; 4]) -> Opcode {
        let v = u32::from_le_bytes(code);
        Opcode(v as u16, (v >> 16) as u16)
    }
    const fn new(class: u16, op: [u8; 2]) -> Self {
        Opcode(class, u16::from_le_bytes(op))
    }

    pub fn class(self) -> u16 {
        self.0
    }
    pub fn op(self) -> u16 {
        self.1
    }

    pub const PING: Self = Self::fourcc(*b"PING");

    /// Asymmetric crypto
    pub const CLASS_ECDSA_P256: u16 = u16::from_le_bytes(*b"P2");
    pub const CLASS_DICE_P256: u16 = u16::from_le_bytes(*b"DI");
    pub const CLASS_ECDH_P256: u16 = u16::from_le_bytes(*b"E2");
    pub const CLASS_ECDSA_P384: u16 = u16::from_le_bytes(*b"P3");
    pub const CLASS_ECDH_P384: u16 = u16::from_le_bytes(*b"E3");
    pub const CLASS_ED25519: u16 = u16::from_le_bytes(*b"Ed");
    pub const CLASS_X25519: u16 = u16::from_le_bytes(*b"X2");
    pub const CLASS_RSA2048: u16 = u16::from_le_bytes(*b"R2");
    pub const CLASS_RSA3072: u16 = u16::from_le_bytes(*b"R3");
    pub const CLASS_RSA4096: u16 = u16::from_le_bytes(*b"R4");

    pub const ECDSA_P256_KEYGEN: Self = Self::new(Self::CLASS_ECDSA_P256, *b"KG");
    pub const ECDSA_P256_SIGN: Self = Self::new(Self::CLASS_ECDSA_P256, *b"SI");
    pub const ECDSA_P256_VERIFY: Self = Self::new(Self::CLASS_ECDSA_P256, *b"VE");
    pub const ECDH_P256_KEYGEN: Self = Self::new(Self::CLASS_ECDH_P256, *b"KG");
    pub const ECDH_P256_KEY_AGREEMENT: Self = Self::new(Self::CLASS_ECDH_P256, *b"AG");

    pub const DICE_P256_KEYGEN: Self = Self::new(Self::CLASS_DICE_P256, *b"KG");
    pub const DICE_P256_SIGN: Self = Self::new(Self::CLASS_DICE_P256, *b"SI");

    pub const ECDSA_P384_KEYGEN: Self = Self::new(Self::CLASS_ECDSA_P384, *b"KG");
    pub const ECDSA_P384_SIGN: Self = Self::new(Self::CLASS_ECDSA_P384, *b"SI");
    pub const ECDSA_P384_VERIFY: Self = Self::new(Self::CLASS_ECDSA_P384, *b"VE");
    pub const ECDH_P384_KEYGEN: Self = Self::new(Self::CLASS_ECDH_P384, *b"KG");
    pub const ECDH_P384_KEY_AGREEMENT: Self = Self::new(Self::CLASS_ECDH_P384, *b"AG");

    pub const ED25519_KEYGEN: Self = Self::new(Self::CLASS_ED25519, *b"KG");
    pub const ED25519_SIGN: Self = Self::new(Self::CLASS_ED25519, *b"SI");
    pub const ED25519_VERIFY: Self = Self::new(Self::CLASS_ED25519, *b"VE");
    pub const X25519_KEYGEN: Self = Self::new(Self::CLASS_X25519, *b"KG");
    pub const X25519_KEY_AGREEMENT: Self = Self::new(Self::CLASS_X25519, *b"AG");

    pub const RSA2048_KEYGEN: Self = Self::new(Self::CLASS_RSA2048, *b"KG");
    pub const RSA2048_SIGN: Self = Self::new(Self::CLASS_RSA2048, *b"SI");
    pub const RSA2048_VERIFY: Self = Self::new(Self::CLASS_RSA2048, *b"VE");

    pub const RSA3072_KEYGEN: Self = Self::new(Self::CLASS_RSA3072, *b"KG");
    pub const RSA3072_SIGN: Self = Self::new(Self::CLASS_RSA3072, *b"SI");
    pub const RSA3072_VERIFY: Self = Self::new(Self::CLASS_RSA3072, *b"VE");

    pub const RSA4096_KEYGEN: Self = Self::new(Self::CLASS_RSA4096, *b"KG");
    pub const RSA4096_SIGN: Self = Self::new(Self::CLASS_RSA4096, *b"SI");
    pub const RSA4096_VERIFY: Self = Self::new(Self::CLASS_RSA4096, *b"VE");

    /// Digest types
    pub const CLASS_SHA2_256: u16 = u16::from_le_bytes(*b"S2");
    pub const CLASS_SHA2_384: u16 = u16::from_le_bytes(*b"S3");
    pub const CLASS_SHA2_512: u16 = u16::from_le_bytes(*b"S5");

    pub const SHA2_256_INIT: Self = Self::new(Self::CLASS_SHA2_256, *b"IN");
    pub const SHA2_256_UPDATE: Self = Self::new(Self::CLASS_SHA2_256, *b"UP");
    pub const SHA2_256_FINAL: Self = Self::new(Self::CLASS_SHA2_256, *b"FI");
    pub const SHA2_384_INIT: Self = Self::new(Self::CLASS_SHA2_384, *b"IN");
    pub const SHA2_384_UPDATE: Self = Self::new(Self::CLASS_SHA2_384, *b"UP");
    pub const SHA2_384_FINAL: Self = Self::new(Self::CLASS_SHA2_384, *b"FI");
    pub const SHA2_512_INIT: Self = Self::new(Self::CLASS_SHA2_512, *b"IN");
    pub const SHA2_512_UPDATE: Self = Self::new(Self::CLASS_SHA2_512, *b"UP");
    pub const SHA2_512_FINAL: Self = Self::new(Self::CLASS_SHA2_512, *b"FI");
}

impl Opcode {
    pub fn as_str(&self) -> &str {
        unsafe {
            let data = core::slice::from_raw_parts(self as *const Opcode as *const u8, 4);
            core::str::from_utf8_unchecked(data)
        }
    }
}

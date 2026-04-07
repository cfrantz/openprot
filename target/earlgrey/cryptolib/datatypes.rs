
use crate::misc::GetPointer;
use crate::otcrypto::*;

/// Possible status values for the cryptolib.
///
/// As long as the OTCRYPTO_STATUS_DEBUG define is unset, all `otcrypto_status_t`
/// codes returned by the cryptolib should be bit-by-bit equivalent with one of
/// the values in this enum.
///
/// Values are built to be bit-compatible with OpenTitan's internal `status_t`
/// datatypes. The highest (sign) bit indicates if the value is an error (1) or
/// not (0). For non-error statuses, the rest can be anything; in cryptolib
/// status codes it is always `kHardenedBoolTrue`. For errors:
///   - The next 15 bits are a module identifier, which is always 0 in the
///     cryptolib status codes
///   - The next 11 bits are a line number or other information; in the
///     cryptolib status codes, it is a hardened value created to have high
///     Hamming distance with the other valid status codes
///   - The final 5 bits are an Abseil-compatible error code
///
/// The hardened values for error codes were generated with:
/// $ ./util/design/sparse-fsm-encode.py -d 5 -m 5 -n 11 \\
///      -s 4232058530 --language=sv --avoid-zero
///
/// Use the same seed value and a larger `-m` argument to generate new values
/// without changing all error codes. Remove the seed (-s argument) to generate
/// completely new 11-bit values.
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
pub struct Error(pub i32);
#[allow(non_upper_case_globals)]
impl Error {
    pub const Ok: Error = Self(1849);
    pub const BadArgs: Error = Self(-2147418461);
    pub const InternalError: Error = Self(-2147462326);
    pub const FatalError: Error = Self(-2147455607);
    pub const AsyncIncomplete: Error = Self(-2147423666);
    pub const NotImplemented: Error = Self(-2147447508);
}

impl From<Error> for i32 {
    fn from(v: Error) -> Self {
        v.0
    }
}
impl GetPointer for Error {
    type Target = i32;
    fn as_ptr(&self) -> *const i32 {
        &self.0 as *const i32
    }
    fn as_mut_ptr(&mut self) -> *mut i32 {
        &mut self.0 as *mut i32
    }
}

pub type CryptoResult = ::core::result::Result<(), Error>;
impl From<otcrypto_status_t> for CryptoResult {
    fn from(sts: otcrypto_status_t) -> Self {
        let e = Error(sts.value);
        if e == Error::Ok {
            Ok(())
        } else {
            Err(e)
        }
    }
}

impl From<Error> for pw_status::Error {
    fn from(e: Error) -> Self {
        unsafe {
            // SAFETY: the low bits of Error are identical
            // to pw_status::Error codes.
            core::mem::transmute(e.0 & 0x1f)
        }
    }
}

/// This is a boolean type for use in hardened contexts.
///
/// The intention is that this is used instead of `<stdbool.h>`'s #bool, where a
/// higher hamming distance is required between the truthy and the falsey value.
///
/// The values below were chosen at random, with some specific restrictions. They
/// have a Hamming Distance of 8, and they are 11-bit values so they can be
/// materialized with a single instruction on RISC-V. They are also specifically
/// not the complement of each other.
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
pub struct HardenedBool(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl HardenedBool {
    pub const True: HardenedBool = Self(1849);
    pub const False: HardenedBool = Self(468);
}

impl From<HardenedBool> for u32 {
    fn from(v: HardenedBool) -> Self {
        v.0
    }
}
impl GetPointer for HardenedBool {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to denote the key type of the handled key.
///
/// Values are hardened.
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
pub struct KeyType(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl KeyType {
    pub const Aes: KeyType = Self(2281);
    pub const Hmac: KeyType = Self(3647);
    pub const Kmac: KeyType = Self(2932);
    pub const Rsa: KeyType = Self(2030);
    pub const Ecc: KeyType = Self(347);
    pub const Kdf: KeyType = Self(2951);
}

impl From<KeyType> for u32 {
    fn from(v: KeyType) -> Self {
        v.0
    }
}
impl GetPointer for KeyType {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to specify the AES modes that use a key.
///
/// This will be used in the `otcrypto_key_mode_t` struct to indicate the mode
/// for which the provided key is intended for.
///
/// Values are hardened.
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
pub struct AesKeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl AesKeyMode {
    pub const Ecb: AesKeyMode = Self(438);
    pub const Cbc: AesKeyMode = Self(3898);
    pub const Cfb: AesKeyMode = Self(249);
    pub const Ofb: AesKeyMode = Self(2889);
    pub const Ctr: AesKeyMode = Self(1230);
    pub const Gcm: AesKeyMode = Self(2725);
    pub const Kwp: AesKeyMode = Self(2005);
}

impl From<AesKeyMode> for u32 {
    fn from(v: AesKeyMode) -> Self {
        v.0
    }
}
impl GetPointer for AesKeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to specify the HMAC modes that use a key.
///
/// This will be used in the `otcrypto_key_mode_t` struct to indicate the mode
/// for which the provided key is intended for.
///
/// Values are hardened.
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
pub struct HmacKeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl HmacKeyMode {
    pub const Sha256: HmacKeyMode = Self(2045);
    pub const Sha384: HmacKeyMode = Self(1083);
    pub const Sha512: HmacKeyMode = Self(1954);
}

impl From<HmacKeyMode> for u32 {
    fn from(v: HmacKeyMode) -> Self {
        v.0
    }
}
impl GetPointer for HmacKeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to specify the KMAC modes that use a key.
///
/// This will be used in the `otcrypto_key_mode_t` struct to indicate the mode
/// for which the provided key is intended for.
///
/// Values are hardened.
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
pub struct KmacKeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl KmacKeyMode {
    pub const Kmac128: KmacKeyMode = Self(2646);
    pub const Kmac256: KmacKeyMode = Self(1635);
}

impl From<KmacKeyMode> for u32 {
    fn from(v: KmacKeyMode) -> Self {
        v.0
    }
}
impl GetPointer for KmacKeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to specify the RSA modes that use a key.
///
/// This will be used in the `otcrypto_key_mode_t` struct to indicate the mode
/// for which the provided key is intended for.
///
/// Values are hardened.
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
pub struct RsaKeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl RsaKeyMode {
    pub const SignPkcs: RsaKeyMode = Self(980);
    pub const SignPss: RsaKeyMode = Self(1889);
    pub const EncryptOaep: RsaKeyMode = Self(1413);
}

impl From<RsaKeyMode> for u32 {
    fn from(v: RsaKeyMode) -> Self {
        v.0
    }
}
impl GetPointer for RsaKeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to specify the ECC modes that use a key.
///
/// This will be used in the `otcrypto_key_mode_t` struct to indicate the mode
/// for which the provided key is intended for.
///
/// Values are hardened.
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
pub struct EccKeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl EccKeyMode {
    pub const EcdsaP256: EccKeyMode = Self(798);
    pub const EcdsaP384: EccKeyMode = Self(1685);
    pub const EcdhP256: EccKeyMode = Self(1532);
    pub const EcdhP384: EccKeyMode = Self(455);
    pub const Ed25519: EccKeyMode = Self(1635);
    pub const X25519: EccKeyMode = Self(187);
}

impl From<EccKeyMode> for u32 {
    fn from(v: EccKeyMode) -> Self {
        v.0
    }
}
impl GetPointer for EccKeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to specify the KDF modes that use a key.
///
/// This will be used in the `otcrypto_key_mode_t` struct to indicate the mode
/// for which the provided key is intended for.
///
/// Values are hardened.
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
pub struct KdfKeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl KdfKeyMode {
    pub const CtrHmac: KdfKeyMode = Self(303);
    pub const Kmac128: KdfKeyMode = Self(3678);
    pub const Kmac256: KdfKeyMode = Self(851);
}

impl From<KdfKeyMode> for u32 {
    fn from(v: KdfKeyMode) -> Self {
        v.0
    }
}
impl GetPointer for KdfKeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum for opentitan crypto modes that use a key.
///
/// Denotes the crypto mode for which the provided key is to be used.
/// This `otcrypto_key_mode_t` will be a parameter in the
/// `otcrypto_blinded_key_t` and `otcrypto_unblinded_key_t` structs.
///
/// Values are hardened.
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
pub struct KeyMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl KeyMode {
    pub const AesEcb: KeyMode = Self(149488054);
    pub const AesCbc: KeyMode = Self(149491514);
    pub const AesCfb: KeyMode = Self(149487865);
    pub const AesOfb: KeyMode = Self(149490505);
    pub const AesCtr: KeyMode = Self(149488846);
    pub const AesGcm: KeyMode = Self(149490341);
    pub const AesKwp: KeyMode = Self(149489621);
    pub const HmacSha256: KeyMode = Self(239011837);
    pub const HmacSha384: KeyMode = Self(239010875);
    pub const HmacSha512: KeyMode = Self(239011746);
    pub const Kmac128: KeyMode = Self(192154198);
    pub const Kmac256: KeyMode = Self(192153187);
    pub const RsaSignPkcs: KeyMode = Self(133039060);
    pub const RsaSignPss: KeyMode = Self(133039969);
    pub const RsaEncryptOaep: KeyMode = Self(133039493);
    pub const EcdsaP256: KeyMode = Self(22741790);
    pub const EcdsaP384: KeyMode = Self(22742677);
    pub const EcdhP256: KeyMode = Self(22742524);
    pub const EcdhP384: KeyMode = Self(22741447);
    pub const Ed25519: KeyMode = Self(22742627);
    pub const X25519: KeyMode = Self(22741179);
    pub const KdfCtrHmac: KeyMode = Self(193397039);
    pub const KdfKmac128: KeyMode = Self(193400414);
    pub const KdfKmac256: KeyMode = Self(193397587);
}

impl From<KeyMode> for u32 {
    fn from(v: KeyMode) -> Self {
        v.0
    }
}
impl GetPointer for KeyMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to denote key security level.
///
/// At high security levels, the crypto library will prioritize
/// protecting the key from sophisticated attacks, even at large
/// performance costs. If the security level is low, the crypto
/// library will still try to protect the key, but may forgo the
/// most costly protections against e.g. sophisticated physical
/// attacks.
///
/// Values are hardened.
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
pub struct KeySecurityLevel(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl KeySecurityLevel {
    pub const Low: KeySecurityLevel = Self(489);
    pub const Medium: KeySecurityLevel = Self(3755);
    pub const High: KeySecurityLevel = Self(2686);
}

impl From<KeySecurityLevel> for u32 {
    fn from(v: KeySecurityLevel) -> Self {
        v.0
    }
}
impl GetPointer for KeySecurityLevel {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to denote the crypto library version.
///
/// In future updates, this enum will be extended to preserve some
/// level of backwards-compatibility despite changes to internal
/// details (for example, the preferred masking scheme for blinded
/// keys).
///
/// Values are hardened.
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
pub struct LibVersion(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl LibVersion {
    pub const _1: LibVersion = Self(2036);
}

impl From<LibVersion> for u32 {
    fn from(v: LibVersion) -> Self {
        v.0
    }
}
impl GetPointer for LibVersion {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to define supported hashing modes.
///
/// Values are hardened.
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
pub struct HashMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl HashMode {
    pub const Sha256: HashMode = Self(1691);
    pub const Sha384: HashMode = Self(1966);
    pub const Sha512: HashMode = Self(369);
    pub const Sha3_224: HashMode = Self(1302);
    pub const Sha3_256: HashMode = Self(724);
    pub const Sha3_384: HashMode = Self(615);
    pub const Sha3_512: HashMode = Self(1101);
    pub const Shake128: HashMode = Self(1496);
    pub const Shake256: HashMode = Self(842);
    pub const Cshake128: HashMode = Self(189);
    pub const Cshake256: HashMode = Self(1250);
}

impl From<HashMode> for u32 {
    fn from(v: HashMode) -> Self {
        v.0
    }
}
impl GetPointer for HashMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to define AES mode of operation.
///
/// Values are hardened.
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
pub struct AesMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl AesMode {
    pub const Ecb: AesMode = Self(1331);
    pub const Cbc: AesMode = Self(1117);
    pub const Cfb: AesMode = Self(3282);
    pub const Ofb: AesMode = Self(922);
    pub const Ctr: AesMode = Self(3372);
}

impl From<AesMode> for u32 {
    fn from(v: AesMode) -> Self {
        v.0
    }
}
impl GetPointer for AesMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to define AES operation to be performed.
///
/// Values are hardened.
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
pub struct AesOperation(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl AesOperation {
    pub const Encrypt: AesOperation = Self(694);
    pub const Decrypt: AesOperation = Self(1520);
}

impl From<AesOperation> for u32 {
    fn from(v: AesOperation) -> Self {
        v.0
    }
}
impl GetPointer for AesOperation {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to define padding scheme for AES data.
///
/// Values are hardened.
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
pub struct AesPadding(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl AesPadding {
    pub const Pkcs7: AesPadding = Self(3711);
    pub const Iso9797M2: AesPadding = Self(4012);
    pub const Null: AesPadding = Self(2254);
}

impl From<AesPadding> for u32 {
    fn from(v: AesPadding) -> Self {
        v.0
    }
}
impl GetPointer for AesPadding {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to denote the AES-GCM tag length.
///
/// Values are hardened.
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
pub struct AesGcmTagLen(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl AesGcmTagLen {
    pub const _128: AesGcmTagLen = Self(359);
    pub const _96: AesGcmTagLen = Self(858);
    pub const _64: AesGcmTagLen = Self(1492);
    pub const _32: AesGcmTagLen = Self(3846);
}

impl From<AesGcmTagLen> for u32 {
    fn from(v: AesGcmTagLen) -> Self {
        v.0
    }
}
impl GetPointer for AesGcmTagLen {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Hashing mode for EdDSA signatures.
///
/// Values are hardened.
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
pub struct EddsaSignMode(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl EddsaSignMode {
    pub const Eddsa: EddsaSignMode = Self(2785);
    pub const HashEddsa: EddsaSignMode = Self(2470);
}

impl From<EddsaSignMode> for u32 {
    fn from(v: EddsaSignMode) -> Self {
        v.0
    }
}
impl GetPointer for EddsaSignMode {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to define padding scheme for RSA signature data.
///
/// Values are hardened.
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
pub struct RsaPadding(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl RsaPadding {
    pub const Pkcs: RsaPadding = Self(2382);
    pub const Pss: RsaPadding = Self(1713);
}

impl From<RsaPadding> for u32 {
    fn from(v: RsaPadding) -> Self {
        v.0
    }
}
impl GetPointer for RsaPadding {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Enum to define possible lengths of RSA (public) keys.
///
/// Values are hardened.
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
pub struct RsaSize(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl RsaSize {
    pub const _2048: RsaSize = Self(1489);
    pub const _3072: RsaSize = Self(3125);
    pub const _4096: RsaSize = Self(2266);
}

impl From<RsaSize> for u32 {
    fn from(v: RsaSize) -> Self {
        v.0
    }
}
impl GetPointer for RsaSize {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        &self.0 as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        &mut self.0 as *mut u32
    }
}

/// Struct to represent the configuration of a blinded key.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct KeyConfig {
    pub version: LibVersion,
    pub key_mode: KeyMode,
    pub key_length: u32,
    pub hw_backed: HardenedBool,
    pub exportable: HardenedBool,
    pub security_level: KeySecurityLevel,
}

impl GetPointer for KeyConfig {
    type Target = otcrypto_key_config;
    fn as_ptr(&self) -> *const otcrypto_key_config {
        self as *const KeyConfig as *const otcrypto_key_config
    }
    fn as_mut_ptr(&mut self) -> *mut otcrypto_key_config {
        self as *mut KeyConfig as *mut otcrypto_key_config
    }
}

impl From<KeyConfig> for otcrypto_key_config {
    fn from(v: KeyConfig) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}

/// Struct to handle unmasked key type.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct UnblindedKey {
    pub key_mode: KeyMode,
    pub key_length: u32,
    pub key: usize,
    pub checksum: u32,
}

impl GetPointer for UnblindedKey {
    type Target = otcrypto_unblinded_key;
    fn as_ptr(&self) -> *const otcrypto_unblinded_key {
        self as *const UnblindedKey as *const otcrypto_unblinded_key
    }
    fn as_mut_ptr(&mut self) -> *mut otcrypto_unblinded_key {
        self as *mut UnblindedKey as *mut otcrypto_unblinded_key
    }
}

impl UnblindedKey {
    pub fn with_key_material(&mut self, km: &[u8]) -> &mut Self {
        // TODO: make sure km is the right size.
        // TODO: Capture km's lifetime and attach it to the returned self reference.
        self.key = km.as_ptr() as usize;
        self
    }
    pub fn with_internal_key_material(&mut self) -> &mut Self {
        let base = &raw const *self as usize;
        self.key = base + core::mem::size_of::<Self>();
        self
    }
}

/// Struct to handle masked key type.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct BlindedKey {
    pub config: KeyConfig,
    pub keyblob_length: u32,
    pub keyblob: usize,
    pub checksum: u32,
}

impl GetPointer for BlindedKey {
    type Target = otcrypto_blinded_key;
    fn as_ptr(&self) -> *const otcrypto_blinded_key {
        self as *const BlindedKey as *const otcrypto_blinded_key
    }
    fn as_mut_ptr(&mut self) -> *mut otcrypto_blinded_key {
        self as *mut BlindedKey as *mut otcrypto_blinded_key
    }
}

impl BlindedKey {
    pub fn with_key_material(&mut self, km: &[u8]) -> &mut Self {
        // TODO: make sure km is the right size.
        // TODO: Capture km's lifetime and attach it to the returned self reference.
        self.keyblob = km.as_ptr() as usize;
        self
    }
    pub fn with_internal_key_material(&mut self) -> &mut Self {
        let base = &raw const *self as usize;
        self.keyblob = base + core::mem::size_of::<Self>();
        self
    }
}

/// Context for a streaming AES-GCM operation.
///
/// Representation is internal to the AES-GCM implementation and subject to
/// change.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct AesGcmContext {
    pub data: [u32; 194usize],
}

impl GetPointer for AesGcmContext {
    type Target = otcrypto_aes_gcm_context;
    fn as_ptr(&self) -> *const otcrypto_aes_gcm_context {
        self as *const AesGcmContext as *const otcrypto_aes_gcm_context
    }
    fn as_mut_ptr(&mut self) -> *mut otcrypto_aes_gcm_context {
        self as *mut AesGcmContext as *mut otcrypto_aes_gcm_context
    }
}

impl From<AesGcmContext> for otcrypto_aes_gcm_context {
    fn from(v: AesGcmContext) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}

/// The DICE diversifier contains two diversification constants.
///
/// - The diversifier is the keymgr 8-word + version diversification constant.
/// - The attestation_seed is additional per-chip fixed entropy that is normally
///   stored in the AttestationKeySeeds flash INFO page (bank=0, page=4).  These
///   constants are 320 bits (10 words) long.  Because OTBN's bignum registers
///   are 256 bits wide, we program a full 512 bits to OTBN.  When you load the
///   attestation seed, load 10 words from flash and set the remaining words to
///   zero.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct DiceKeymgrDiversifier {
    pub salt: [u32; 8usize],
    pub version: u32,
}

impl GetPointer for DiceKeymgrDiversifier {
    type Target = dice_keymgr_diversifier;
    fn as_ptr(&self) -> *const dice_keymgr_diversifier {
        self as *const DiceKeymgrDiversifier as *const dice_keymgr_diversifier
    }
    fn as_mut_ptr(&mut self) -> *mut dice_keymgr_diversifier {
        self as *mut DiceKeymgrDiversifier as *mut dice_keymgr_diversifier
    }
}

impl From<DiceKeymgrDiversifier> for dice_keymgr_diversifier {
    fn from(v: DiceKeymgrDiversifier) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}

#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct DiceDiversifier {
    pub diversifier: DiceKeymgrDiversifier,
    pub attestation_seed: [u32; 16usize],
}

impl GetPointer for DiceDiversifier {
    type Target = dice_diversifier;
    fn as_ptr(&self) -> *const dice_diversifier {
        self as *const DiceDiversifier as *const dice_diversifier
    }
    fn as_mut_ptr(&mut self) -> *mut dice_diversifier {
        self as *mut DiceDiversifier as *mut dice_diversifier
    }
}

impl From<DiceDiversifier> for dice_diversifier {
    fn from(v: DiceDiversifier) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}

/// Opaque SHA-2 hash context.
///
/// Representation is internal to the hash implementation; initialize
/// with #otcrypto_sha2_init.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct Sha2Context {
    pub data: [u32; 88usize],
}

impl GetPointer for Sha2Context {
    type Target = otcrypto_sha2_context;
    fn as_ptr(&self) -> *const otcrypto_sha2_context {
        self as *const Sha2Context as *const otcrypto_sha2_context
    }
    fn as_mut_ptr(&mut self) -> *mut otcrypto_sha2_context {
        self as *mut Sha2Context as *mut otcrypto_sha2_context
    }
}

impl From<Sha2Context> for otcrypto_sha2_context {
    fn from(v: Sha2Context) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}

/// Generic hmac context.
///
/// Representation is internal to the hmac implementation; initialize
/// with #otcrypto_hmac_init.
#[derive(
    Debug,
    Clone,
    zerocopy::KnownLayout,
    zerocopy::Immutable,
    zerocopy::FromBytes,
    zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct HmacContext {
    pub data: [u32; 88usize],
}

impl GetPointer for HmacContext {
    type Target = otcrypto_hmac_context;
    fn as_ptr(&self) -> *const otcrypto_hmac_context {
        self as *const HmacContext as *const otcrypto_hmac_context
    }
    fn as_mut_ptr(&mut self) -> *mut otcrypto_hmac_context {
        self as *mut HmacContext as *mut otcrypto_hmac_context
    }
}

impl From<HmacContext> for otcrypto_hmac_context {
    fn from(v: HmacContext) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}

#[derive(
    Debug, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes,
)]
#[repr(C)]
pub struct HashDigest {
    pub mode: HashMode,
    pub digest: [u32],
}

impl From<&HashDigest> for otcrypto_hash_digest {
    fn from(v: &HashDigest) -> Self {
        otcrypto_hash_digest {
            mode: v.mode.into(),
            data: v.digest.as_ptr() as *mut u32,
            len: v.digest.len(),
        }
    }
}
impl From<&mut HashDigest> for otcrypto_hash_digest {
    fn from(v: &mut HashDigest) -> Self {
        otcrypto_hash_digest {
            mode: v.mode.into(),
            data: v.digest.as_mut_ptr(),
            len: v.digest.len(),
        }
    }
}

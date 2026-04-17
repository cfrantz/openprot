#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]

use app_crypto_user::handle;
use pw_status::{Result, StatusCode, Error};
use userspace::entry;
use userspace::syscall;
use util_misc::hexdump;
use zerocopy::IntoBytes;

use crypto_traits::NoParam;
use crypto_traits::error::ErrorType;
use crypto_traits::backend::Backend;
use crypto_traits::drbg::Drbg;
use crypto_traits::Algorithm;
use crypto_traits::asymmetric::{
    HardwareKey as _,
    Algorithm as EcdsaAlgorithm, AlgoParams,
    KeyPairGen, Sign, Verify,
    EcdsaP256,
    ShareSecret,
};
use crypto_traits::symmetric::{
    Aes128,
    AlgoParams as _,
    SymmetricOp,
};
use crypto_common::CipherMode;
use crypto_common::symmetric_key::{
    Aes128Key,
    KeyMode,
};
use crypto_traits::digest::{
    Digest,
    Sha2_256,
    Sha2_384,
    Algorithm as DigestAlgorithm,
    DigestInit, DigestUpdate, DigestFinal,
};
use crypto_traits::hmac::{
    HmacInit, HmacUpdate, HmacFinal,
    HmacSha256,
};

use crypto_common::keytypes::{EcdsaP256PrivateKey, EcdsaP256PublicKey, EcdsaP256Signature
};
use crypto_client::backend::CryptoClient;
use crypto_client::ecdsa::{SoftwareKey, HardwareKey};
use crypto_client::hmac::HmacSha256Key;
use crypto_client::sha2::Sha2_256Digest;

const GETTYSBURG_PRELUDE: &'static str = "\
Four score and seven years ago our fathers brought forth on this \
continent, a new nation, conceived in Liberty, and dedicated to the \
proposition that all men are created equal.";

const GETTYSBURG_DIGEST: Sha2_256Digest = Sha2_256Digest::new([
    0x1e, 0x6f, 0xd4, 0x03, 0x0f, 0x90, 0x34, 0xcd, 0x77, 0x57, 0x08, 0xa3, 0x96, 0xc3, 0x24, 0xed,
    0x42, 0x0e, 0xc5, 0x87, 0xeb, 0x3d, 0xd4, 0x33, 0xe2, 0x9f, 0x6a, 0xc0, 0x8b, 0x8c, 0xc7, 0xba,
]);

const P256_N: [u32;8] = [0xfc632551, 0xf3b9cac2, 0xa7179e84, 0xbce6faad,
                         0xffffffff, 0xffffffff, 0x00000000, 0xffffffff];


fn sha_digest_test<Handle, A, D>(handle: &Handle, alg: &A, data: &[u8]) -> Result<D>
where
    Handle: Backend + ErrorType<Error=Error>,
    A: DigestAlgorithm<Handle>,
    Handle: DigestInit<A>,
    Handle: DigestUpdate<<Handle as DigestInit<A>>::Context>,
    Handle: DigestFinal<<Handle as DigestInit<A>>::Context, Digest=D>,
{
    let ctx = handle.init(alg)?;
    handle.update(&ctx, data)?;
    handle.finalize(ctx)
}


fn p256_sign_test<Handle, D>(handle: &Handle, digest: &D) -> Result<EcdsaP256Signature>
where
    D: Digest,
    Handle: Backend + ErrorType<Error=Error>,
    Handle: KeyPairGen<EcdsaP256, SoftwareKey>,
    Handle: Sign<EcdsaP256PrivateKey, Message=D, Param=NoParam, Signature=EcdsaP256Signature>,
    Handle: Verify<EcdsaP256PublicKey, Message=D, Param=NoParam, Signature=EcdsaP256Signature>,
    EcdsaP256: EcdsaAlgorithm<Handle>,
    SoftwareKey: AlgoParams<Handle, EcdsaP256, PrivateKey=EcdsaP256PrivateKey, PublicKey=EcdsaP256PublicKey>,
{
    pw_log::info!("Ecdsa keygen");
    let (private, public) = handle.key_pair_gen(&EcdsaP256, &SoftwareKey)?;

    let sig = handle.sign(&private, digest, &NoParam)?;

    let verify = handle.verify(&public, digest, &NoParam, &sig)?;
    pw_log::info!("verify: {}", verify as bool);

    Ok(sig)
}

fn sum(data: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];

    /*
    util_misc::add_mod(
        &data[..32],
        &data[40..72],
        P256_N.as_bytes(),
        &mut output,
    );
    */
    for i in 0..32 {
        output[i] = data[i] ^ data[i+40];
    }
    output
}

fn p256_share_test(handle: &CryptoClient) -> Result<()> {
    let (ska, pka) = handle.key_pair_gen(&EcdsaP256, &SoftwareKey)?;
    let (skb, pkb) = handle.key_pair_gen(&EcdsaP256, &SoftwareKey)?;

    let sa = handle.share_secret(&ska, &pkb)?;
    let sb = handle.share_secret(&skb, &pka)?;

    let aa = sum(&sa.data);
    let bb = sum(&sb.data);

    hexdump(&aa);
    hexdump(&bb);
    Ok(())
}

fn hmac_test<'a, Handle, A, D>(handle: &Handle, alg: &A, key: &[u8], data: &[u8]) -> Result<D>
where
    Handle: Backend + ErrorType<Error = Error> + HmacInit<'a, A>, //, Key = HmacSha256Key>,
    A: Algorithm<Handle>,
    Handle: HmacUpdate<<Handle as HmacInit<'a, A>>::Context>,
    Handle: HmacFinal<<Handle as HmacInit<'a, A>>::Context, Tag = D>,
    D: Digest,
{
    let ctx = handle.hmac_init(alg, key)?;
    handle.hmac_update(&ctx, data)?;
    handle.hmac_finalize(ctx)
}


fn drbg_test<Handle>(handle: &Handle) -> Result<()>
where
    Handle: Drbg + ErrorType<Error = Error>,
{
    pw_log::info!("DRBG instantiate");
    handle.instantiate(&[1, 2, 3, 4])?;

    let mut buf = [0u8; 32];
    pw_log::info!("DRBG generate");
    handle.generate(&[], &mut buf)?;
    hexdump(&buf);

    pw_log::info!("DRBG reseed");
    handle.reseed(&[5, 6, 7, 8])?;

    pw_log::info!("DRBG generate");
    handle.generate(&[], &mut buf)?;
    hexdump(&buf);

    pw_log::info!("DRBG uninstantiate");
    handle.uninstantiate()?;

    Ok(())
}

fn aes_test(handle: &CryptoClient, keymode: KeyMode, mode: CipherMode) -> Result<()> {
    let key = Aes128Key::with_key_material(keymode, b"0123456789abcdef");
    let mut iv = [0u8; 16];
    let data = b"This was a test.And this was too";

    pw_log::info!("AES encrypt (keymode={:08x})", u32::from(keymode) as u32);
    let mut buf1 = [0u8; 32];
    handle.encrypt(&mode, &key, &mut iv, data, &mut buf1)?;
    hexdump(&buf1);

    pw_log::info!("AES decrypt:");
    let mut iv = [0u8; 16];
    let mut buf2 = [0u8; 32];
    handle.decrypt(&mode, &key, &mut iv, &buf1, &mut buf2)?;
    hexdump(&buf2);

    Ok(())
}

fn generic_test(handle: &CryptoClient) -> Result<()> {
    let digest = sha_digest_test(handle, &Sha2_256, GETTYSBURG_PRELUDE.as_bytes())?;
    pw_log::info!("Generic: got digest");
    hexdump(&digest);

    let sig = p256_sign_test(handle, &digest)?;
    pw_log::info!("Generic: got signature");
    hexdump(&sig);
    Ok(())
}

fn add_mod_tests() {
    let mut res = [0u32; 8];
    let five = [5u32, 0,0,0,0,0,0,0];
    let two = [2u32, 0,0,0,0,0,0,0];

    util_misc::add_mod(five.as_bytes(), two.as_bytes(), P256_N.as_bytes(), res.as_mut_bytes());
    hexdump(&res);
}

fn test(handle: &CryptoClient) -> Result<()> {
    //drbg_test(handle)?;

    //aes_test(handle, KeyMode::AesEcb, CipherMode::ECB)?;
    //aes_test(handle, KeyMode::AesCbc, CipherMode::CBC)?;
    //aes_test(handle, KeyMode::AesCfb, CipherMode::CFB)?;
    //aes_test(handle, KeyMode::AesOfb, CipherMode::OFB)?;
    //aes_test(handle, KeyMode::AesCtr, CipherMode::CTR)?;

    //let hmac_tag = hmac_test(handle, &HmacSha256, &[0u8; 32], GETTYSBURG_PRELUDE.as_bytes())?;
    //let hmac_tag = hmac_test(handle, &HmacSha256, &[0x31, 0x31, 0x31, 0x31], GETTYSBURG_PRELUDE.as_bytes())?;
    //pw_log::info!("HMAC tag:");
    //hexdump(&hmac_tag);

    add_mod_tests();
    p256_share_test(handle)?;
    generic_test(handle)?;


    let ctx = handle.init(&Sha2_256)?;
    handle.update(&ctx, GETTYSBURG_PRELUDE.as_bytes())?;
    let digest = handle.finalize(ctx)?;

    pw_log::info!("Got digest");
    hexdump(&digest);

    pw_log::info!("Wanted digest");
    hexdump(&GETTYSBURG_DIGEST);

    ////////////////////////////////////////////////////////////
    // ECDSA P-256 keygen/sign/verify
    ////////////////////////////////////////////////////////////
    pw_log::info!("Ecdsa keygen");
    let (private, public) = handle.key_pair_gen(&EcdsaP256, &SoftwareKey)?;
    //let (private, public) = handle.key_pair_gen(&EcdsaP256, &HardwareKey::new([ 1,2,3,4,5,6,7 ]))?;
    pw_log::info!("private:");
    hexdump(&private);
    /*
    let mut z = [0u8; 32];
    util_misc::add_mod(
        &private.data[..32],
        &private.data[40..72],
        P256_N.as_bytes(),
        &mut z,
    );
    */
    let z = sum(&private.data);
    hexdump(&z);

    pw_log::info!("public:");
    hexdump(&public);

    let digest = &GETTYSBURG_DIGEST;
    let sig = handle.sign(&private, digest, &NoParam)?;
    pw_log::info!("signature:");
    hexdump(&sig);

    let verify = handle.verify(&public, digest, &NoParam, &sig)?;
    pw_log::info!("verify: {}", verify as bool);

    Ok(())
}

#[entry]
fn entry() -> ! {
    let c = CryptoClient::new(handle::CRYPTOLIB);
    let ret = test(&c);

    pw_log::error!("crypto user ended with {}", ret.status_code() as u32);
    // Since this is written as a test, shut down with the return status from `main()`.
    let _ = syscall::debug_shutdown(ret);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    pw_log::error!("crypto user panic");
    loop {}
}

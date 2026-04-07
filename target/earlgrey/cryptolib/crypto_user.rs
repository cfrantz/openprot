#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]

use app_crypto_user::handle;
use pw_status::{Result, StatusCode, Error};
use userspace::entry;
use userspace::syscall;
//use userspace::time::Instant;
use util_misc::hexdump;

use crypto_traits::NoParam;
use crypto_traits::error::ErrorType;
use crypto_traits::backend::Backend;
use crypto_traits::asymmetric::{
    Algorithm, AlgoParams,
    KeyPairGen, Sign, Verify,
    EcdsaP256,
    //SoftwareKey as _,
};
use crypto_traits::digest::{
    Digest,
    Sha2_256,
    Sha2_384,
    Algorithm as DigestAlgorithm,
    DigestInit, DigestUpdate, DigestFinal,
};

use crypto_common::keytypes::{EcdsaP256PrivateKey, EcdsaP256PublicKey, EcdsaP256Signature
};
use crypto_client::backend::CryptoClient;
use crypto_client::ecdsa::SoftwareKey;
use crypto_client::sha2::Sha2_256Digest;

const GETTYSBURG_PRELUDE: &'static str = "\
Four score and seven years ago our fathers brought forth on this \
continent, a new nation, conceived in Liberty, and dedicated to the \
proposition that all men are created equal.";

const GETTYSBURG_DIGEST: Sha2_256Digest = Sha2_256Digest::new([
    0x1e, 0x6f, 0xd4, 0x03, 0x0f, 0x90, 0x34, 0xcd, 0x77, 0x57, 0x08, 0xa3, 0x96, 0xc3, 0x24, 0xed,
    0x42, 0x0e, 0xc5, 0x87, 0xeb, 0x3d, 0xd4, 0x33, 0xe2, 0x9f, 0x6a, 0xc0, 0x8b, 0x8c, 0xc7, 0xba,
]);

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
    EcdsaP256: Algorithm<Handle>,
    SoftwareKey: AlgoParams<Handle, EcdsaP256, PrivateKey=EcdsaP256PrivateKey, PublicKey=EcdsaP256PublicKey>,
{
    pw_log::info!("Ecdsa keygen");
    let (private, public) = handle.key_pair_gen(&EcdsaP256, &SoftwareKey)?;

    //pw_log::info!("digest type: {}", core::any::type_name_of_val(digest) as &str);
    //let x = &GETTYSBURG_DIGEST;
    //pw_log::info!("digest type: {}", core::any::type_name_of_val(x) as &str);
    let sig = handle.sign(&private, digest, &NoParam)?;

    let verify = handle.verify(&public, digest, &NoParam, &sig)?;
    pw_log::info!("verify: {}", verify as bool);

    Ok(sig)
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

fn test(handle: &CryptoClient) -> Result<()> {

    generic_test(handle)?;


    let ctx = handle.init(&Sha2_256)?;
    handle.update(&ctx, GETTYSBURG_PRELUDE.as_bytes())?;
    let digest = handle.finalize(ctx)?;

    pw_log::info!("Got digest");
    hexdump(&digest);

    pw_log::info!("Wanted digest");
    hexdump(&GETTYSBURG_DIGEST);

    // Generics test

    ////////////////////////////////////////////////////////////
    // ECDSA P-256 keygen/sign/verify
    ////////////////////////////////////////////////////////////
    pw_log::info!("Ecdsa keygen");
    let (private, public) = handle.key_pair_gen(&EcdsaP256, &SoftwareKey)?;
    pw_log::info!("private:");
    hexdump(&private);
    pw_log::info!("public:");
    hexdump(&public);

    let digest = &GETTYSBURG_DIGEST;
    let sig = handle.sign(&private, digest, &NoParam)?;
    pw_log::info!("signature:");
    hexdump(&sig);

    let verify = handle.verify(&public, digest, &NoParam, &sig)?;
    pw_log::info!("verify: {}", verify as bool);

    ////////////////////////////////////////////////////////////
    // RSA2048 keygen/sign/verify
    ////////////////////////////////////////////////////////////
//    pw_log::info!("Rsa keygen");
//    let (private, public) = handle.key_pair_gen(Rsa, &RsaSize3072(RsaKeyMode::SignPkcs))?;
//    pw_log::info!("private:");
//    hexdump(&private);
//    pw_log::info!("public:");
//    hexdump(&public);
//
//    let digest = &GETTYSBURG_DIGEST;
//    let sig = handle.sign(&private, digest, &RsaPadding::Pkcs)?;
//    pw_log::info!("signature:");
//    hexdump(&sig);
//
//    let verify = handle.verify(&public, digest, &RsaPadding::Pkcs, &sig)?;
//    pw_log::info!("verify: {}", verify as bool);


//
//    //let (akey, asig) = handle.cdi1_attest(&GETTYSBURG_DIGEST)?;
//    //let mut buf = [0u8; 128];
//    //pw_log::info!("amsg: {}", hexstr(&mut buf, &GETTYSBURG_DIGEST));
//    //pw_log::info!("akey: {}", hexstr(&mut buf, &akey.keyblob));
//    //pw_log::info!("asig: {}", hexstr(&mut buf, &asig));
//
    Ok(())
}

//fn doit(c: &CryptoClient) -> Result<()> {
//    let mut code = 0u32;
//    let len = c.transaction(&[&Opcode::PING], &mut[&mut code], Instant::MAX)?;
//    pw_log::info!("transaction: code={:x}, len={}", code as u32, len as usize);
//    Ok(())
//}

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

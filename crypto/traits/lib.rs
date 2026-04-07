#![no_std]

use backend::Backend;
pub mod aes;
pub mod asymmetric;
pub mod backend;
pub mod digest;
pub mod drbg;
pub mod error;
pub mod hmac;
pub mod keymgr;

/// The algorithm trait represents an algorithm that the backend can perform.
pub trait Algorithm<B: Backend + ?Sized> {}

#[derive(Clone, Copy)]
pub struct NoParam;

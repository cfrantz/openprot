#![no_std]
pub mod asymmetric;
pub mod backend;
pub mod digest;
pub mod error;

#[derive(Clone, Copy)]
pub struct NoParam;

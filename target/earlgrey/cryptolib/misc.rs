
use crate::otcrypto::*;

pub(crate) trait GetPointer {
    type Target;
    fn as_ptr(&self) -> *const Self::Target;
    fn as_mut_ptr(&mut self) -> *mut Self::Target;
}

impl GetPointer for u8 {
    type Target = u8;
    fn as_ptr(&self) -> *const u8 {
        self as *const u8
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self as *mut u8
    }
}

impl GetPointer for u16 {
    type Target = u16;
    fn as_ptr(&self) -> *const u16 {
        self as *const u16
    }
    fn as_mut_ptr(&mut self) -> *mut u16 {
        self as *mut u16
    }
}

impl GetPointer for u32 {
    type Target = u32;
    fn as_ptr(&self) -> *const u32 {
        self as *const u32
    }
    fn as_mut_ptr(&mut self) -> *mut u32 {
        self as *mut u32
    }
}

impl GetPointer for u64 {
    type Target = u64;
    fn as_ptr(&self) -> *const u64 {
        self as *const u64
    }
    fn as_mut_ptr(&mut self) -> *mut u64 {
        self as *mut u64
    }
}

impl GetPointer for usize {
    type Target = usize;
    fn as_ptr(&self) -> *const usize {
        self as *const usize
    }
    fn as_mut_ptr(&mut self) -> *mut usize {
        self as *mut usize
    }
}

impl GetPointer for i8 {
    type Target = i8;
    fn as_ptr(&self) -> *const i8 {
        self as *const i8
    }
    fn as_mut_ptr(&mut self) -> *mut i8 {
        self as *mut i8
    }
}

impl GetPointer for i16 {
    type Target = i16;
    fn as_ptr(&self) -> *const i16 {
        self as *const i16
    }
    fn as_mut_ptr(&mut self) -> *mut i16 {
        self as *mut i16
    }
}

impl GetPointer for i32 {
    type Target = i32;
    fn as_ptr(&self) -> *const i32 {
        self as *const i32
    }
    fn as_mut_ptr(&mut self) -> *mut i32 {
        self as *mut i32
    }
}

impl GetPointer for i64 {
    type Target = i64;
    fn as_ptr(&self) -> *const i64 {
        self as *const i64
    }
    fn as_mut_ptr(&mut self) -> *mut i64 {
        self as *mut i64
    }
}

impl GetPointer for isize {
    type Target = isize;
    fn as_ptr(&self) -> *const isize {
        self as *const isize
    }
    fn as_mut_ptr(&mut self) -> *mut isize {
        self as *mut isize
    }
}

impl From<&mut [u8]> for otcrypto_byte_buf {
    fn from(buf: &mut [u8]) -> Self {
        Self {
            data: buf.as_mut_ptr(),
            len: buf.len(),
        }
    }
}
impl From<&[u8]> for otcrypto_const_byte_buf {
    fn from(buf: &[u8]) -> Self {
        Self {
            data: buf.as_ptr(),
            len: buf.len(),
        }
    }
}

// TODO: I'm cheating and accepting a u8 slice here.
impl From<&mut [u8]> for otcrypto_word32_buf {
    fn from(buf: &mut [u8]) -> Self {
        Self {
            data: buf.as_mut_ptr() as *mut u32,
            len: buf.len() / 4,
        }
    }
}
impl From<&[u8]> for otcrypto_const_word32_buf {
    fn from(buf: &[u8]) -> Self {
        Self {
            data: buf.as_ptr() as *const u32,
            len: buf.len() / 4,
        }
    }
}

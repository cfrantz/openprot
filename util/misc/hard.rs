// Modular add for recovering blinded private keys.

unsafe extern "C" {
    fn hardened_add_mod(x: *const u8, y: *const u8, n: *const u8, word_len: usize, dest: *mut u8);
}

pub fn add_mod(x: &[u8], y: &[u8], n: &[u8], dest: &mut [u8]) {
    assert_eq!(x.len(), y.len());
    assert_eq!(x.len(), n.len());
    assert_eq!(x.len(), dest.len());
    unsafe {
        hardened_add_mod(
            x.as_ptr(),
            y.as_ptr(),
            n.as_ptr(),
            dest.len() / 4,
            dest.as_mut_ptr(),
        )
    }
}

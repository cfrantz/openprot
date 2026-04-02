use zerocopy::{Immutable, IntoBytes};

const HEX: [u8; 16] = *b"0123456789ABCDEF";

pub fn hexdump<T>(data: &T)
where
    T: IntoBytes + Immutable + ?Sized,
{
    let data = data.as_bytes();
    for (i, d) in data.chunks(16).enumerate() {
        let mut buf = [b' '; 76];
        let mut offset = i * 16;
        for j in 0..8 {
            buf[7 - j] = HEX[offset & 15];
            offset = offset >> 4;
        }
        for (j, &byte) in d.iter().enumerate() {
            buf[10 + j * 3] = HEX[(byte >> 4) as usize];
            buf[11 + j * 3] = HEX[(byte & 15) as usize];
            buf[60 + j] = if byte >= 0x20 && byte < 0x7f {
                byte
            } else {
                b'.'
            };
        }
        let line = unsafe { core::str::from_utf8_unchecked(&buf[..60 + d.len()]) };
        pw_log::info!("{}", line as &str);
    }
}

pub fn hexstr<'a, T>(dest: &'a mut [u8], data: &T) -> &'a str
where
    T: IntoBytes + Immutable + ?Sized,
{
    let data = data.as_bytes();
    let mut i = 0;
    for &byte in data.iter() {
        dest[i] = HEX[(byte >> 4) as usize];
        dest[i + 1] = HEX[(byte & 15) as usize];
        i += 2;
    }
    unsafe { core::str::from_utf8_unchecked(&dest[..i]) }
}

/*
const GETTYSBURG_PRELUDE: &'static str = "\
Four score and seven years ago our fathers brought forth on this \
continent, a new nation, conceived in Liberty, and dedicated to the \
proposition that all men are created equal.";

const GETTYSBURG_DIGEST: [u8; 32] = [
    0x1e, 0x6f, 0xd4, 0x03, 0x0f, 0x90, 0x34, 0xcd, 0x77, 0x57, 0x08, 0xa3, 0x96, 0xc3, 0x24, 0xed,
    0x42, 0x0e, 0xc5, 0x87, 0xeb, 0x3d, 0xd4, 0x33, 0xe2, 0x9f, 0x6a, 0xc0, 0x8b, 0x8c, 0xc7, 0xba,
];

fn main() {
    let buf = [0u8, 1,2,3,4,5,100,128,160];
    println!("buf:");
    hexdump(&buf);

    println!("Gettysburg prelude:");
    hexdump(GETTYSBURG_PRELUDE);

    println!("Gettysburg digest:");
    hexdump(&GETTYSBURG_DIGEST);
}
*/

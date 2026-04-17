pub struct Crc32(u32);

impl Crc32 {
    const POLY: u32 = 0xedb88320;
    const MU: u32 = 0xf7011641;

    pub fn new() -> Self {
        Crc32(u32::MAX)
    }

    fn crc32_i(x: u32) -> u32 {
        let mut result;
        unsafe {
            core::arch::asm!("
            .option push;
            .option arch, +zbc;
            clmul {result}, {x}, {mu};
            clmulr {result}, {result}, {poly};
            .option pop;
        ",
            result = out(reg) result,
            x = in(reg) x,
            mu = in(reg) Self::MU,
            poly = in(reg) Self::POLY,
            )
        };
        result
    }

    pub fn add32(&mut self, word: u32) {
        self.0 = Self::crc32_i(self.0 ^ word);
    }

    pub fn add8(&mut self, byte: u8) {
        let ctx = self.0 ^ (byte as u32);
        self.0 = Self::crc32_i(ctx << 24) ^ (ctx >> 8);
    }

    pub fn add(&mut self, data: &[u8]) {
        for &byte in data.iter() {
            self.add8(byte);
        }
    }

    pub fn finalize(self) -> u32 {
        self.0 ^ u32::MAX
    }
}

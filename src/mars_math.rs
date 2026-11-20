pub fn wrap(val: i64, core_size: u32) -> u32 {
    let m = core_size as i64;
    ((val % m) + m) as u32 % core_size
}

pub fn add_mod(a: u32, b: u32, core_size: u32) -> u32 {
    ((a as u64 + b as u64) % core_size as u64) as u32
}

pub fn sub_mod(a: u32, b: u32, core_size: u32) -> u32 {
    (a + core_size - (b % core_size)) % core_size
}

pub fn mul_mod(a: u32, b: u32, core_size: u32) -> u32 {
    ((a as u64 * b as u64) % core_size as u64) as u32
}

pub fn inc_mod(a: u32, core_size: u32) -> u32 {
    let next = a + 1;
    if next >= core_size { 0 } else { next }
}

pub fn dec_mod(a: u32, core_size: u32) -> u32 {
    if a == 0 { core_size - 1 } else { a - 1 }
}

pub fn to_signed(val: u32, core_size: u32) -> i32 {
    if val > core_size / 2 {
        val as i32 - core_size as i32
    } else {
        val as i32
    }
}

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let s = if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed };
        Self { state: s }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 16) as u32
    }

    pub fn gen_range(&mut self, bound: u32) -> u32 {
        if bound == 0 { 0 } else { self.next_u32() % bound }
    }
}

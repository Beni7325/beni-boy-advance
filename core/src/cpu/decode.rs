pub const fn extract_bits(data: u32, high_bit: u8, n_bits: u8) -> u32 {

    let mask = (1 << n_bits) - 1;
    let shift_left = high_bit + 1 - n_bits;

    (data >> shift_left) & mask
}

pub const fn match_opcode_pattern(opcode: u32, pattern: &str) -> bool {

    let mut mask = 0;
    let mut target = 0;

    let bytes = pattern.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'0' => {
                mask = (mask << 1) | 1;
                target = target << 1;
            },
            b'1' => {
                mask = (mask << 1) | 1;
                target = (target << 1) | 1;
            },
            b'.' => {
                mask = mask << 1;
                target = target << 1;
            },
            _ => panic!()
        }

        i += 1;
    }

    opcode & mask == target
}

#[macro_export]
macro_rules! fill_dispatch {
    (size: $size:expr, default: $default:expr, $($pattern:expr => $func:expr),* $(,)?) => {{
        let mut table = [$default as fn(&mut Arm7Tdmi, _); $size];
        let mut i = 0u32;
        while i < $size as u32 {
            $(
                if match_opcode_pattern(i, $pattern) {
                    table[i as usize] = $func;
                    i += 1;
                    continue;
                }
            )*
            i += 1;
        }
        table
    }};
}

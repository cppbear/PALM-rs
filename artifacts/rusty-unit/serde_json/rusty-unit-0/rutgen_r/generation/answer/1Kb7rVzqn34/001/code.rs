// Answer 0

const fn decode_hex_val_slow(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

const fn build_hex_table(shift: usize) -> [i16; 256] {
    let mut table = [0; 256];
    let mut ch = 0;
    while ch < 256 {
        table[ch] = match decode_hex_val_slow(ch as u8) {
            Some(val) => (val as i16) << shift,
            None => -1,
        };
        ch += 1;
    }
    table
}

#[test]
fn test_build_hex_table_valid() {
    let table = build_hex_table(1);
    for ch in 0..=0x0F {
        assert_eq!(table[ch], (ch as i16) << 1);
    }
}

#[test]
fn test_build_hex_table_invalid() {
    let table = build_hex_table(1);
    for ch in 0x10..=0xFF {
        assert_eq!(table[ch], -1);
    }
}

#[test]
fn test_build_hex_table_boundary() {
    let table = build_hex_table(1);
    let ch = 256;
    assert_eq!(table[ch & 0xFF], -1); // Accessing index at the boundary
}


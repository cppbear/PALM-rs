// Answer 0

const fn decode_hex_val_slow(value: u8) -> Option<u8> {
    match value {
        0..=9 => Some(value), // For '0' to '9'
        10..=15 => Some(value + 39), // For 'A' to 'F' (capital)
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
fn test_build_hex_table_all_valid_cases() {
    let result = build_hex_table(0);
    assert_eq!(result[0], 0);   // '0' -> 0 << 0
    assert_eq!(result[1], 1);   // '1' -> 1 << 0
    assert_eq!(result[2], 2);   // '2' -> 2 << 0
    assert_eq!(result[3], 3);   // '3' -> 3 << 0
    assert_eq!(result[4], 4);   // '4' -> 4 << 0
    assert_eq!(result[5], 5);   // '5' -> 5 << 0
    assert_eq!(result[6], 6);   // '6' -> 6 << 0
    assert_eq!(result[7], 7);   // '7' -> 7 << 0
    assert_eq!(result[8], 8);   // '8' -> 8 << 0
    assert_eq!(result[9], 9);   // '9' -> 9 << 0
    assert_eq!(result[10], 10 << 0); // 'A' -> 10 << 0
    assert_eq!(result[11], 11 << 0); // 'B' -> 11 << 0
    assert_eq!(result[12], 12 << 0); // 'C' -> 12 << 0
    assert_eq!(result[13], 13 << 0); // 'D' -> 13 << 0
    assert_eq!(result[14], 14 << 0); // 'E' -> 14 << 0
    assert_eq!(result[15], 15 << 0); // 'F' -> 15 << 0
    for i in 16..256 {
        assert_eq!(result[i], -1); // All other values
    }
}

#[test]
fn test_build_hex_table_with_shift() {
    let result = build_hex_table(1);
    assert_eq!(result[0], 0);   
    assert_eq!(result[1], 1);   
    assert_eq!(result[2], 2);   
    assert_eq!(result[3], 3);   
    assert_eq!(result[4], 4);   
    assert_eq!(result[5], 5);   
    assert_eq!(result[6], 6);   
    assert_eq!(result[7], 7);   
    assert_eq!(result[8], 8);   
    assert_eq!(result[9], 9);   
    assert_eq!(result[10], 20);  // 'A' -> 10 << 1
    assert_eq!(result[11], 22);  // 'B' -> 11 << 1
    assert_eq!(result[12], 24);  // 'C' -> 12 << 1
    assert_eq!(result[13], 26);  // 'D' -> 13 << 1
    assert_eq!(result[14], 28);  // 'E' -> 14 << 1
    assert_eq!(result[15], 30);  // 'F' -> 15 << 1
    for i in 16..256 {
        assert_eq!(result[i], -1);
    }
}

#[test]
#[should_panic]
fn test_build_hex_table_panic_on_ch_exceeding_bound() {
    let result = build_hex_table(0);
    let _ = result[256]; // Induces panic since index 256 is out of bounds
}


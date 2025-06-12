// Answer 0

const fn decode_hex_val_slow(val: u8) -> Option<u8> {
    if val < 16 {
        Some(val)
    } else {
        None
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
fn test_build_hex_table_zero_shift() {
    let result = build_hex_table(0);
    assert_eq!(result[0], 0);
    assert_eq!(result[1], 1);
    assert_eq!(result[2], 2);
    assert_eq!(result[3], 3);
    assert_eq!(result[15], 15);
    assert_eq!(result[16], -1);
    assert_eq!(result[255], -1);
}

#[test]
fn test_build_hex_table_positive_shift() {
    let result = build_hex_table(1);
    assert_eq!(result[0], 0);
    assert_eq!(result[1], 2);
    assert_eq!(result[2], 4);
    assert_eq!(result[3], 6);
    assert_eq!(result[15], 30);
    assert_eq!(result[16], -1);
    assert_eq!(result[255], -1);
}

#[test]
fn test_build_hex_table_max_shift() {
    let result = build_hex_table(10);
    assert_eq!(result[0], 0);
    assert_eq!(result[1], 1024);
    assert_eq!(result[2], 2048);
    assert_eq!(result[3], 3072);
    assert_eq!(result[15], 30720);
    assert_eq!(result[16], -1);
    assert_eq!(result[255], -1);
}

#[test]
#[should_panic]
fn test_build_hex_table_out_of_bounds() {
    let result = build_hex_table(256);
    assert_eq!(result[255], -1);
}


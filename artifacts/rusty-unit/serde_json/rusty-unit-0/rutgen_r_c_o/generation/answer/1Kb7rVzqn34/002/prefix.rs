// Answer 0

#[test]
fn test_build_hex_table_with_shift_zero() {
    let _table = build_hex_table(0);
}

#[test]
fn test_build_hex_table_with_shift_one() {
    let _table = build_hex_table(1);
}

#[test]
fn test_build_hex_table_with_shift_two() {
    let _table = build_hex_table(2);
}

#[test]
fn test_build_hex_table_with_shift_three() {
    let _table = build_hex_table(3);
}

#[test]
fn test_build_hex_table_with_shift_four() {
    let _table = build_hex_table(4);
}

#[test]
#[should_panic]
fn test_build_hex_table_with_ch_at_bound() {
    let mut table = [0; 256];
    let ch = 256;
    table[ch] = match decode_hex_val_slow(ch as u8) {
        Some(val) => (val as i16) << 0,
        None => -1,
    };
}


// Answer 0

#[test]
fn test_build_hex_table_ch_equals_256() {
    let shift = 0;
    let table = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_ch_equals_256_with_shift() {
    let shift = 4;
    let table = build_hex_table(shift);
}


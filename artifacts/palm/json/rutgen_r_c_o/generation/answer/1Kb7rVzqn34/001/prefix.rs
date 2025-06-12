// Answer 0

#[test]
fn test_build_hex_table_zero_shift() {
    let table = build_hex_table(0);
}

#[test]
fn test_build_hex_table_four_shift() {
    let table = build_hex_table(4);
}

#[test]
fn test_build_hex_table_boundary() {
    let table = build_hex_table(1);
}


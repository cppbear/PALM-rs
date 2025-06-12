// Answer 0

#[test]
fn test_build_hex_table_valid_cases() {
    const SHIFT: usize = 0;
    let table = build_hex_table(SHIFT);
    assert_eq!(table[0], 0);
    assert_eq!(table[1], 1);
    assert_eq!(table[2], 2);
    assert_eq!(table[3], 3);
    assert_eq!(table[10], 10);
    assert_eq!(table[15], 15);
    assert_eq!(table[16], -1); // Expected for '10'
    assert_eq!(table[255], -1); // Expected for any non-hex character
}

#[test]
fn test_build_hex_table_shifted_cases() {
    const SHIFT: usize = 4;
    let table = build_hex_table(SHIFT);
    assert_eq!(table[0], 0);
    assert_eq!(table[1], 16);
    assert_eq!(table[2], 32);
    assert_eq!(table[3], 48);
    assert_eq!(table[10], 160);
    assert_eq!(table[15], 240);
    assert_eq!(table[16], -16); // Expected for '10'
    assert_eq!(table[255], -1); // Expected for any non-hex character
}

#[test]
fn test_build_hex_table_max_bound() {
    const SHIFT: usize = 0;
    let table = build_hex_table(SHIFT);
    assert_eq!(table[256], 0); // Out of bounds behavior; should not panic but can be handled gracefully in further functions.
}


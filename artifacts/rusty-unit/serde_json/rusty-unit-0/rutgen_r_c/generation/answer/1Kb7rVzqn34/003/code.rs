// Answer 0

#[test]
fn test_build_hex_table_edge_case() {
    const SHIFT: usize = 0; // Testing with a shift of 0 for specification
    let result = build_hex_table(SHIFT);
    // Validate that all values must be zero since no hex decoding is performed
    for &value in &result {
        assert_eq!(value, 0);
    }
}

#[test]
fn test_build_hex_table_with_shift() {
    const SHIFT: usize = 4; // Testing with a shift of 4
    let result = build_hex_table(SHIFT);
    
    // Check specific hex values to ensure correct left shifting
    assert_eq!(result[b'0' as usize], 0 << SHIFT); // '0' -> 0
    assert_eq!(result[b'1' as usize], 1 << SHIFT); // '1' -> 1 shifted by 4
    assert_eq!(result[b'2' as usize], 2 << SHIFT); // '2' -> 2 shifted by 4
    assert_eq!(result[b'9' as usize], 9 << SHIFT); // '9' -> 9 shifted by 4
    assert_eq!(result[b'A' as usize], 10 << SHIFT); // 'A' -> 10 shifted by 4
    assert_eq!(result[b'F' as usize], 15 << SHIFT); // 'F' -> 15 shifted by 4
    assert_eq!(result[b'a' as usize], 10 << SHIFT); // 'a' -> 10 shifted by 4
    assert_eq!(result[b'f' as usize], 15 << SHIFT); // 'f' -> 15 shifted by 4
    
    // Check out of range values
    for ch in 0..=255 {
        if !matches!(ch, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f') {
            assert_eq!(result[ch as usize], -1); // All other values should map to -1
        }
    }
}


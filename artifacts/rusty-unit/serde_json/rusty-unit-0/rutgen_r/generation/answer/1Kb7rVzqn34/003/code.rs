// Answer 0

#[test]
fn test_build_hex_table_boundary_condition() {
    const SHIFT: usize = 0; // Choosing a shift of 0 to verify basic functionality
    let result = build_hex_table(SHIFT);
    
    // Since we are considering when `ch < 256` is false, we won't have an entry for `ch == 256`
    // All values in the result should be -1 for each ch that cannot decode a hex value
    for i in 0..256 {
        let expected = if let Some(_) = decode_hex_val_slow(i as u8) {
            (i as i16) << SHIFT
        } else {
            -1
        };
        assert_eq!(result[i], expected, "Mismatch at index: {}", i);
    }
} 

#[test]
fn test_build_hex_table_with_shift() {
    const SHIFT: usize = 1; // Testing with a shift of 1
    let result = build_hex_table(SHIFT);
    
    for i in 0..256 {
        let expected = if let Some(_) = decode_hex_val_slow(i as u8) {
            (i as i16) << SHIFT
        } else {
            -1
        };
        assert_eq!(result[i], expected, "Mismatch at index: {}", i);
    }
} 

#[test]
fn test_build_hex_table_with_large_shift() {
    const SHIFT: usize = 10; // Testing with a large shift value
    let result = build_hex_table(SHIFT);
    
    for i in 0..256 {
        let expected = if let Some(_) = decode_hex_val_slow(i as u8) {
            (i as i16) << SHIFT
        } else {
            -1
        };
        assert_eq!(result[i], expected, "Mismatch at index: {}", i);
    }
}


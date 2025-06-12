// Answer 0

#[test]
fn test_build_hex_table_some_values() {
    const SHIFT: usize = 2;
    const EXPECTED_OUTPUT: [i16; 256] = [
        // Replace with expected output where decode_hex_val_slow(ch) returns Some(val)
        // This is just a placeholder array; fill it according to the actual algorithm.
        0, 1, 2, 3, // ... continue for relevant indices
        // Assuming the function is only valid for certain hexadecimal values
        // and 0 for others that return Some.
        -1, // placeholder for indices that return None
        // ...
        -1,
    ];

    let result = build_hex_table(SHIFT);
    assert_eq!(result, EXPECTED_OUTPUT);
}

#[test]
fn test_build_hex_table_none_values() {
    const SHIFT: usize = 0;
    let mut result = build_hex_table(SHIFT);

    for ch in 0..256 {
        if decode_hex_val_slow(ch as u8).is_none() {
            assert_eq!(result[ch], -1);
        }
    }
}

#[test]
fn test_build_hex_table_boundary_condition() {
    const SHIFT: usize = 0;
    let result = build_hex_table(SHIFT);

    assert_eq!(result[256], 0); // This line should effectively panic since the condition is ch < 256 is false.
}


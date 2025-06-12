// Answer 0

#[test]
fn test_build_hex_table_zero_shift() {
    const EXPECTED: [i16; 256] = [
        0, 1, 2, 3, 4, 5, 6, 7, 
        8, 9, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1,
        // ... complete for all values
        -1, -1, -1, 10, 11, 12, 13, 14, 
        15, -1, -1, -1, -1, -1, -1, -1
    ];
    
    let table = build_hex_table(0);
    assert_eq!(table, EXPECTED);
}

#[test]
fn test_build_hex_table_four_shift() {
    const EXPECTED: [i16; 256] = [
        0, 16, 32, 48, 64, 80, 96, 112, 
        128, 144, -16, -16, -16, -16, -16, -16,
        -16, -16, -16, -16, -16, -16, -16, -16,
        -16, -16, -16, -16, -16, -16, -16, -16,
        -16, -16, -16, -16, -16, -16, -16, -16,
        // ... complete for all values
        -16, -16, -16, 160, 176, 192, 208, 224, 
        240, -16, -16, -16, -16, -16, -16, -16
    ];
    
    let table = build_hex_table(4);
    assert_eq!(table, EXPECTED);
}


// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let input = &[0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x41] = 0; // A
        table[0x42] = 1; // B
        table[0x43] = 2; // C
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x46] = 5; // F
        table[0x47] = 6; // G
        table[0x48] = 7; // H
        table
    };
    let mut output = [0; 6];
    decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output).unwrap();
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input = &[0x41, 0x42, 0xFF, 0x44, 0x45, 0x46, 0x47, 0x48];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x41] = 0; // A
        table[0x42] = 1; // B
        // 0xFF is invalid, so remains INVALID_VALUE
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x46] = 5; // F
        table[0x47] = 6; // G
        table[0x48] = 7; // H
        table
    };
    let mut output = [0; 6];
    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
}


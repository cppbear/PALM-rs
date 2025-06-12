// Answer 0

#[test]
fn test_decode_chunk_8_with_invalid_byte_at_6() {
    let input: &[u8] = &[0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xFF, 0x47];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x41] = 0; // A
        table[0x42] = 1; // B
        table[0x43] = 2; // C
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x46] = 5; // F
        // Ensure 0xFF leads to INVALID_VALUE
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let _result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_with_invalid_byte_at_start() {
    let input: &[u8] = &[0xFF, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x42] = 1; // B
        table[0x43] = 2; // C
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x46] = 5; // F
        // Make sure 0x41 is a valid value 
        table[0x41] = 0; // A
        // Ignore the rest
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let _result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_with_all_valid_bytes_except_last() {
    let input: &[u8] = &[0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xFF, 0x49];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x41] = 0; // A
        table[0x42] = 1; // B
        table[0x43] = 2; // C
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x46] = 5; // F
        // Ensure 0xFF leads to INVALID_VALUE
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let _result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_with_all_invalid() {
    let input: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // No valid values
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let _result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
}


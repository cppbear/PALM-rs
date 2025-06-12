// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_at_index_4() {
    let input = [0x41, 0x42, 0x43, 0x44, 0xFF, 0x45, 0x46, 0x47];
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
        // Ensure valid mappings for A-G and not for 0xFF
        table
    };
    let mut output = [0; 6];
    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_after_valid_bytes() {
    let input = [0x42, 0x43, 0x44, 0x45, 0xFF, 0x47, 0x48, 0x49];
    let index_at_start_of_input = 1;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x42] = 1; // B
        table[0x43] = 2; // C
        table[0x44] = 3; // D
        table[0x45] = 4; // E
        table[0x47] = 6; // G
        table[0x48] = 7; // H
        table[0x49] = 8; // I
        // Ensure valid mappings for B-E, G-I and not for 0xFF
        table
    };
    let mut output = [0; 6];
    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_all_valid_bytes() {
    let input = [0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48]; // A-h
    let index_at_start_of_input = 2;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0u8..=0x3f {
            table[i + 0x41] = i; // A-Z
        }
        for i in 0u8..=0x1f {
            table[i + 0x61] = i + 26; // a-z
        }
        table
    };
    let mut output = [0; 6];
    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_partial_valid_with_error() {
    let input = [0x4D, 0x5A, 0x3A, 0x4F, 0xFF, 0x4C, 0x52, 0x54]; // MZ:OLRT among others
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x4D] = 12; // M
        table[0x5A] = 25; // Z
        table[0x3A] = INVALID_VALUE; // :
        table[0x4F] = 14; // O
        table[0x4C] = 11; // L
        table[0x52] = 17; // R
        table[0x54] = 19; // T
        // Ensure a mix of valid and invalid bytes, with an invalid byte
        table
    };
    let mut output = [0; 6];
    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}


// Answer 0

#[test]
fn test_decode_chunk_4_valid_input() {
    let input: [u8; 4] = [65, 66, 67, 68]; // Corresponding to valid base64 symbols
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[65] = 0; // A
        table[66] = 1; // B
        table[67] = 2; // C
        table[68] = 3; // D
        table
    };
    let mut output: [u8; 3] = [0; 3];
    let index_at_start_of_input = 0;
    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte() {
    let input: [u8; 4] = [65, 66, 255, 68]; // 255 is not a valid base64 symbol
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[65] = 0; // A
        table[66] = 1; // B
        table[68] = 3; // D
        table
    };
    let mut output: [u8; 3] = [0; 3];
    let index_at_start_of_input = 0;
    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_at_second_index() {
    let input: [u8; 4] = [65, 255, 67, 68]; // 255 is not a valid base64 symbol
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[65] = 0; // A
        table[67] = 2; // C
        table[68] = 3; // D
        table
    };
    let mut output: [u8; 3] = [0; 3];
    let index_at_start_of_input = 1;
    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_at_third_index() {
    let input: [u8; 4] = [65, 66, 255, 68]; // 255 is not a valid base64 symbol
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[65] = 0; // A
        table[66] = 1; // B
        table[68] = 3; // D
        table
    };
    let mut output: [u8; 3] = [0; 3];
    let index_at_start_of_input = 2;
    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_at_fourth_index() {
    let input: [u8; 4] = [65, 66, 67, 255]; // 255 is not a valid base64 symbol
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[65] = 0; // A
        table[66] = 1; // B
        table[67] = 2; // C
        table
    };
    let mut output: [u8; 3] = [0; 3];
    let index_at_start_of_input = 3;
    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}


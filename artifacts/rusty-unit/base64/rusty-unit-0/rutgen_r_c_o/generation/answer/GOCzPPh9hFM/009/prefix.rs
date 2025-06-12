// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let input: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in 0..8 {
            table[i] = i; // Mapping these values directly to themselves
        }
        table
    };
    let mut output: [u8; 6] = [0; 6];

    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_first() {
    let input: [u8; 8] = [0xFF, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0x01] = 1; 
        table[0x02] = 2; 
        table[0x03] = 3; 
        table[0x04] = 4; 
        table[0x05] = 5; 
        table[0x06] = 6; 
        table[0x07] = 7; 
        table
    };
    let mut output: [u8; 6] = [0; 6];

    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_last() {
    let input: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0xFF];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0x00] = 0; 
        table[0x01] = 1; 
        table[0x02] = 2; 
        table[0x03] = 3; 
        table[0x04] = 4; 
        table[0x05] = 5; 
        table[0x06] = 6; 
        table[0x07] = 7; 
        table
    };
    let mut output: [u8; 6] = [0; 6];

    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_all_invalid() {
    let input: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table.fill(INVALID_VALUE);
        table
    };
    let mut output: [u8; 6] = [0; 6];

    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}


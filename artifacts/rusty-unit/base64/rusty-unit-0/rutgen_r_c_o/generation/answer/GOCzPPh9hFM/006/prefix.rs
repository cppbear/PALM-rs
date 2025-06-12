// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input: [u8; 8] = [0x41, 0x42, 0x43, 0x44, 0x45, 0xFF, 0x46, 0x47];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x41] = 0; // 'A'
        table[0x42] = 1; // 'B'
        table[0x43] = 2; // 'C'
        table[0x44] = 3; // 'D'
        table[0x45] = 4; // 'E'
        table[0x46] = 5; // 'F'
        table[0x47] = 6; // 'G'
        table // Other entries remain INVALID_VALUE
    };
    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}


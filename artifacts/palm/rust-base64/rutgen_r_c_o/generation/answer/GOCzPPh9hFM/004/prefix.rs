// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input = &[b'A', b'B', b'C', b'%', b'D', b'E', b'F', b'G'];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        // Fill with valid values for A, B, C
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        // Fill with INVALID_VALUE for "%"
        table[b'%' as usize] = INVALID_VALUE;
        // Fill with valid values for D, E, F, G
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table
    };
    let mut output = [0; 6];
    let _ = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
}


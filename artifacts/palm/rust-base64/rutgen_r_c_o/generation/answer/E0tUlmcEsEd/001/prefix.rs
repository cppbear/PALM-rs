// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_0() {
    let input: &[u8] = &[INVALID_VALUE, 0, 0, 0];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 3] = [0; 3];
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_1() {
    let input: &[u8] = &[0, INVALID_VALUE, 0, 0];
    let index_at_start_of_input: usize = 1;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 3] = [0; 3];
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_2() {
    let input: &[u8] = &[0, 0, INVALID_VALUE, 0];
    let index_at_start_of_input: usize = 2;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 3] = [0; 3];
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_3() {
    let input: &[u8] = &[0, 0, 0, INVALID_VALUE];
    let index_at_start_of_input: usize = 3;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 3] = [0; 3];
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}


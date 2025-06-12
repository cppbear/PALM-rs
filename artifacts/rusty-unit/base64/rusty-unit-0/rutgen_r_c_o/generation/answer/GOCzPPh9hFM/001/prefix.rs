// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_case_1() {
    let input: [u8; 8] = [255, 0, 0, 0, 0, 0, 0, 0];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_2() {
    let input: [u8; 8] = [0, 255, 0, 0, 0, 0, 0, 0];
    let index_at_start_of_input: usize = 1;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_3() {
    let input: [u8; 8] = [0, 0, 255, 0, 0, 0, 0, 0];
    let index_at_start_of_input: usize = 2;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_4() {
    let input: [u8; 8] = [0, 0, 0, 255, 0, 0, 0, 0];
    let index_at_start_of_input: usize = 3;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_5() {
    let input: [u8; 8] = [0, 0, 0, 0, 255, 0, 0, 0];
    let index_at_start_of_input: usize = 4;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_6() {
    let input: [u8; 8] = [0, 0, 0, 0, 0, 255, 0, 0];
    let index_at_start_of_input: usize = 5;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_7() {
    let input: [u8; 8] = [0, 0, 0, 0, 0, 0, 255, 0];
    let index_at_start_of_input: usize = 6;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}

#[test]
fn test_decode_chunk_8_invalid_byte_case_8() {
    let input: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 255];
    let index_at_start_of_input: usize = 7;
    let decode_table: [u8; 256] = [0; 256];
    let mut output: [u8; 6] = [0; 6];
    decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output).unwrap_err();
}


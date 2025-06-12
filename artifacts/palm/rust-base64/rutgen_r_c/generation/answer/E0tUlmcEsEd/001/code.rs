// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_first() {
    let input: &[u8] = &[255, b'A', b'B', b'C']; // 255 is invalid, should return an error for this byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Set all to INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[0])));
}

#[test]
fn test_decode_chunk_4_invalid_byte_second() {
    let input: &[u8] = &[b'A', 255, b'B', b'C']; // 255 is invalid, should return an error for this byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Set all to INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1])));
}

#[test]
fn test_decode_chunk_4_invalid_byte_third() {
    let input: &[u8] = &[b'A', b'B', 255, b'C']; // 255 is invalid, should return an error for this byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Set all to INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2])));
}

#[test]
fn test_decode_chunk_4_invalid_byte_fourth() {
    let input: &[u8] = &[b'A', b'B', b'C', 255]; // 255 is invalid, should return an error for this byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Set all to INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3])));
}


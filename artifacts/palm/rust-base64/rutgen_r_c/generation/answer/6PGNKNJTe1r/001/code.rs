// Answer 0

#[test]
fn test_complete_quads_len_invalid_byte_error() {
    let input: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05]; // Length is 5 (input.len() % 4 == 1)
    let input_len_rem: usize = 1;
    let output_len: usize = 3; // Sufficient output length for complete quads
    let mut decode_table: [u8; 256] = [0; 256];
    decode_table[0x05] = INVALID_VALUE; // Set an invalid value for the last_byte

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(4, 0x05))));
}

#[test]
fn test_complete_quads_len_output_slice_too_small() {
    let input: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05]; // Length is 5 (input.len() % 4 == 1)
    let input_len_rem: usize = 1;
    let output_len: usize = 2; // Insufficient output length for complete quads
    let mut decode_table: [u8; 256] = [0; 256];
    decode_table[0x04] = INVALID_VALUE; // Last byte is valid since it is not 0x05

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_complete_quads_len_valid_case() {
    let input: &[u8] = &[0x01, 0x02, 0x03, 0x04]; // Length is 4 (input.len() % 4 == 0)
    let input_len_rem: usize = 0;
    let output_len: usize = 3; // Sufficient output length
    let decode_table: [u8; 256] = [0; 256]; // All bytes valid

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(4)); // Valid case should return Ok(4)
}


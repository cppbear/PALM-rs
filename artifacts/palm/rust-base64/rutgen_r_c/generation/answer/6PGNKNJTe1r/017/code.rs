// Answer 0

#[test]
fn test_complete_quads_len_invalid_input_length() {
    let input: &[u8] = b"abcd";
    let input_len_rem = 1; // This doesn't match with input.len() % 4 which is 0.
    let output_len = 6;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert!(result.is_err());
}

#[test]
fn test_complete_quads_len_invalid_byte() {
    let input: &[u8] = b"abcd\x80"; // Invalid byte outside the Base64 alphabet.
    let input_len_rem = 1; // input.len() % 4 == 1
    let output_len = 6;
    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[usize::from(b'a')] = 0; // Populate valid decode table values for valid Base64 chars
    decode_table[usize::from(b'b')] = 1;
    decode_table[usize::from(b'c')] = 2;
    decode_table[usize::from(b'd')] = 3;

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(5, 0x80)))));
}

#[test]
fn test_complete_quads_len_output_slice_too_small() {
    let input: &[u8] = b"abcde"; // Valid Base64 input
    let input_len_rem = 1; // input.len() % 4 == 1
    let output_len = 1; // Output length is too small
    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[usize::from(b'a')] = 0; // Populate valid decode table values
    decode_table[usize::from(b'b')] = 1;
    decode_table[usize::from(b'c')] = 2;
    decode_table[usize::from(b'd')] = 3;
    decode_table[usize::from(b'e')] = 4;

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert!(matches!(result, Err(DecodeSliceError::OutputSliceTooSmall)));
}

#[test]
fn test_complete_quads_len_valid_case() {
    let input: &[u8] = b"abcd"; // A complete quads input without any issues.
    let input_len_rem = 0; // input.len() % 4 == 0
    let output_len = 3; // Allocated output length
    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[usize::from(b'a')] = 0;
    decode_table[usize::from(b'b')] = 1;
    decode_table[usize::from(b'c')] = 2;
    decode_table[usize::from(b'd')] = 3;

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(4));
}


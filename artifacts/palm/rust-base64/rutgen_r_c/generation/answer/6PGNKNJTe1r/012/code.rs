// Answer 0

#[test]
fn test_complete_quads_len_output_slice_too_small() {
    const DECODE_TABLE: [u8; 256] = [INVALID_VALUE; 256];
    let input: &[u8] = b"YWF"; // Length is 3, hence `len() % 4 == 1`
    let input_len_rem = 1; // input.len() % 4 == 1
    let output_len = 0; // output_len < input_complete_nonterminal_quads_len / 4 * 3 should trigger the error

    let result = complete_quads_len(input, input_len_rem, output_len, &DECODE_TABLE);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_complete_quads_len_invalid_byte() {
    const DECODE_TABLE: [u8; 256] = {
        let mut arr = [INVALID_VALUE; 256];
        arr[usize::from(b'A')] = 0;  // Valid character
        arr[usize::from(b'Y')] = 1;  // Valid character
        arr
    };
    let input: &[u8] = b"YWFQ"; // Length is 4, but input_len_rem will be 0
    let input_len_rem = 0; // input.len() % 4 == 0

    let output_len = 1; // This will pass as there's no condition for Err here
    let result = complete_quads_len(input, input_len_rem, output_len, &DECODE_TABLE);
    assert_eq!(result, Ok(4)); // Should return the length of input
}

#[test]
fn test_complete_quads_len_invalid_surrounding_byte() {
    const DECODE_TABLE: [u8; 256] = {
        let mut arr = [INVALID_VALUE; 256];
        arr[usize::from(b'A')] = 0;  // Valid character
        arr[usize::from(b'Y')] = 1;  // Valid character
        arr
    };
    let input: &[u8] = b"YWFd"; // Last byte 'd' (invalid) will cause an error
    let input_len_rem = 1; // input.len() % 4 == 1
    let output_len = 2; // This will be too small as we have 4 bytes we need to handle

    let result = complete_quads_len(input, input_len_rem, output_len, &DECODE_TABLE);
    assert_eq!(result, Err(DecodeError::InvalidByte(3, b'd').into())); // Invalid byte
}


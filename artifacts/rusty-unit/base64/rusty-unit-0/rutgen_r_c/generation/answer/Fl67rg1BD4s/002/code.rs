// Answer 0

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = &[0xFF, PAD_BYTE, PAD_BYTE, 0x00]; // 4 bytes, with padding at the start
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3]; // output buffer
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // empty decode table for testing

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, DecodePaddingMode::Indifferent);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(1, PAD_BYTE))));
}

#[test]
fn test_decode_suffix_invalid_padding_with_two_characters() {
    let input: &[u8] = &[PAD_BYTE, PAD_BYTE, 0xAC, 0x00]; // 4 bytes, with padding at the start
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3]; // output buffer
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // empty decode table for testing

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, DecodePaddingMode::Indifferent);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(0, PAD_BYTE))));
}


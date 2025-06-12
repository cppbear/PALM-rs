// Answer 0

#[test]
fn test_decode_suffix_invalid_byte_padding() {
    let input: &[u8] = &[0x00, 0x01, 0x02, 0x03]; // Length is 4
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Fill parts of the decode table for valid base64 (for example)
        table[b'A' as usize] = 0; // Assuming 'A' decodes to 0
        table[b'B' as usize] = 1; // Assuming 'B' decodes to 1
        // ... continue filling for other valid characters
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(input_index + 0, 0x00))));
}

#[test]
fn test_decode_suffix_invalid_byte_non_base64() {
    let input: &[u8] = &[0x80, 0x81, 0x82, 0x83]; // Length is 4
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Simulating that higher values are invalid
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(input_index + 0, 0x80))));
}


// Answer 0

#[test]
fn test_decode_suffix_valid_input() {
    let input: &[u8] = &[0b11000011, 0b11011100];
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // should be filled appropriately for real decoding
    decode_table[b'A' as usize] = 0; // Assuming 'A' decodes to 0
    decode_table[b'=' as usize] = PAD_BYTE; // Assuming padding as defined
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_ok());
}

#[test]
fn test_decode_suffix_invalid_input_too_short() {
    let input: &[u8] = &[];
    let input_index = 0;
    let mut output = [0u8; 0];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // should be filled appropriately for real decoding
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = &[b'A', b'B', b'C', b'=', b'='];
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // should be filled appropriately for real decoding
    decode_table[b'A' as usize] = 0; // Assuming 'A' decodes to 0
    decode_table[b'B' as usize] = 1; // Assuming 'B' decodes to 1
    decode_table[b'C' as usize] = 2; // Assuming 'C' decodes to 2
    decode_table[b'=' as usize] = PAD_BYTE; // Padding byte

    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_decode_suffix_invalid_last_symbol() {
    let input: &[u8] = &[b'A', b'B', b'C', b'Z']; // Assuming 'Z' is invalid
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; // Assuming 'A' decodes to 0
    decode_table[b'B' as usize] = 1; // Assuming 'B' decodes to 1
    decode_table[b'C' as usize] = 2; // Assuming 'C' decodes to 2
    // 'Z' remains INVALID_VALUE

    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(3, b'Z'))));
}


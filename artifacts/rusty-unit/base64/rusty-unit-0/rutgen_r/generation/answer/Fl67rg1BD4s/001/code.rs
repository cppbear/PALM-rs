// Answer 0

#[test]
fn test_decode_suffix_invalid_byte_with_padding_index_0() {
    let input: &[u8] = b"////"; // 4 bytes of padding
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All INVALID_VALUE
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 0);
        assert_eq!(byte, b'/');
    } else {
        panic!("Expected Err(DecodeError::InvalidByte) but got {:?}", result);
    }
}

#[test]
fn test_decode_suffix_invalid_byte_with_padding_index_1() {
    let input: &[u8] = b"//AA"; // 3 valid bytes followed by padding
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All INVALID_VALUE
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 1); // the index of the first padding byte
        assert_eq!(byte, b'/');
    } else {
        panic!("Expected Err(DecodeError::InvalidByte) but got {:?}", result);
    }
} 

#[test]
fn test_decode_suffix_invalid_length_with_single_byte() {
    let input: &[u8] = b"A"; // 1 valid byte
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All INVALID_VALUE
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    if let Err(DecodeError::InvalidLength(index)) = result {
        assert_eq!(index, 1); // the index of the invalid length
    } else {
        panic!("Expected Err(DecodeError::InvalidLength) but got {:?}", result);
    }
} 

#[test]
fn test_decode_suffix_invalid_last_symbol() {
    let input: &[u8] = b"AA/"; // Valid byte, but invalid last symbol
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All INVALID_VALUE
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    if let Err(DecodeError::InvalidLastSymbol(index, byte)) = result {
        assert_eq!(index, 2); // the index of the last symbol
        assert_eq!(byte, b'/'); // the last symbol itself
    } else {
        panic!("Expected Err(DecodeError::InvalidLastSymbol) but got {:?}", result);
    }
} 

#[test]
fn test_decode_suffix_output_slice_too_small() {
    let input: &[u8] = b"AAAB"; // 4 valid bytes
    let input_index = 0;
    let mut output = [0u8; 2]; // Output slice is too small
    let output_index = 0;
    let decode_table = [INVALID_VALUE; 256]; // All INVALID_VALUE
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    assert!(matches!(result, Err(DecodeSliceError::OutputSliceTooSmall)));
}


// Answer 0

#[test]
fn test_decode_suffix_invalid_byte_padding() {
    let input: &[u8] = b"ABCD==="; // 4 valid base64 symbols followed by invalid padding
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3]; // small output buffer
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256 // using the invalid value for all bytes
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(4, b'='))));
}

#[test]
fn test_decode_suffix_invalid_byte_non_padding() {
    let input: &[u8] = b"ABCDxyz"; // 4 valid symbols followed by invalid base64 symbols
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3]; // small output buffer
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256 // all invalid for this case
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(4, b'y'))));
}

#[test]
fn test_decode_suffix_invalid_length() {
    let input: &[u8] = b"AB"; // only 2 valid symbols which is insufficient
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3]; // ensure we have a reasonable output buffer
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256 // same invalid setup
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidLength(2))));
}


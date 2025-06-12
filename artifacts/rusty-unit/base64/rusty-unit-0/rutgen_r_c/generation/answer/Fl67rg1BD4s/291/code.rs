// Answer 0

#[test]
fn test_decode_suffix_input_too_long() {
    let input: &[u8] = b"testdata"; // Length is greater than 4
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_decode_suffix_input_index_out_of_bounds() {
    let input: &[u8] = b"test"; // Valid input
    let input_index = 5; // Out of bounds
    let mut output: [u8; 4] = [0; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode).unwrap();
}

#[test]
fn test_decode_suffix_extraneous_padding() {
    let input: &[u8] = b"test=="; // Trailing padding = 
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index = 0;
    let decode_table = {
        let mut dt = [INVALID_VALUE; 256];
        dt[b't' as usize] = 19; // valid value
        dt[b'e' as usize] = 4;  // valid value
        dt[b's' as usize] = 18; // valid value
        dt[b'=' as usize] = INVALID_VALUE; // This should trigger an invalid byte error
        dt
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());
}

#[test]
fn test_decode_suffix_valid_input() {
    let input: &[u8] = b"AA=="; // Valid base64 input
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index = 0;
    let decode_table = {
        let mut dt = [INVALID_VALUE; 256];
        dt[b'A' as usize] = 0; // valid value
        dt[b'=' as usize] = INVALID_VALUE; // padding character
        dt
    };
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 1);
    assert_eq!(metadata.padding_offset, Some(input_index + 2));
}


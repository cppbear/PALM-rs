// Answer 0

#[test]
fn test_decode_suffix_valid_padding() {
    let input: &[u8] = &[65, 66, 67, 68]; // base64 encoded "ABCD"
    let input_index = 0;
    let mut output: [u8; 3] = [0; 3]; // expects to decode 3 bytes.
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 3);
    assert!(metadata.padding_offset.is_none());
}

#[test]
fn test_decode_suffix_with_padding() {
    let input: &[u8] = &[65, 66, 67, b'=']; // base64 encoded "ABC="
    let input_index = 0;
    let mut output: [u8; 3] = [0; 3]; // expects to decode 3 bytes.
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'=' as usize] = INVALID_VALUE; // treat '=' as padding
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 3);
    assert!(metadata.padding_offset.is_some());
}

#[test]
fn test_decode_suffix_invalid_byte() {
    let input: &[u8] = &[65, 66, 255, 68]; // invalid byte 255
    let input_index = 0;
    let mut output: [u8; 3] = [0; 3]; // expects to decode 3 bytes.
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    if let DecodeSliceError::DecodeError(DecodeError::InvalidByte(offset, byte)) = result.unwrap_err() {
        assert_eq!(offset, 2);
        assert_eq!(byte, 255);
    }
}

#[test]
fn test_decode_suffix_invalid_length() {
    let input: &[u8] = &[65]; // only one byte "A"
    let input_index = 0;
    let mut output: [u8; 3] = [0; 3];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    if let DecodeSliceError::DecodeError(DecodeError::InvalidLength(offset)) = result.unwrap_err() {
        assert_eq!(offset, 1);
    }
}

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = &[65, 66, 67, 68, b'=']; // "ABCD="
    let input_index = 0;
    let mut output: [u8; 3] = [0; 3];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'=' as usize] = INVALID_VALUE; // treat '=' as padding
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    if let DecodeSliceError::DecodeError(DecodeError::InvalidPadding) = result.unwrap_err() {
    } else {
        panic!("Expected an invalid padding error");
    }
}


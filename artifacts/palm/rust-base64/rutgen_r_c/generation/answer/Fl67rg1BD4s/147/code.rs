// Answer 0

#[test]
fn test_decode_suffix_invalid_byte_with_padding() {
    let input: &[u8] = &b"QUJD="[..]; // Last character is a padding byte
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; // 0
    decode_table[b'Q' as usize] = 16; // 16
    decode_table[b'J' as usize] = 9; // 9
    decode_table[b'D' as usize] = 3; // 3
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(
            input_index + 4 - 1, b'='
        ))),
    );
}

#[test]
fn test_decode_suffix_invalid_byte_after_morsel() {
    let input: &[u8] = &b"QUJD\xFF"[..]; // Invalid byte after valid base64
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; // 0
    decode_table[b'Q' as usize] = 16; // 16
    decode_table[b'J' as usize] = 9; // 9
    decode_table[b'D' as usize] = 3; // 3
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

    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(
            input_index + 4 - 1, b'\xFF'
        ))),
    );
}


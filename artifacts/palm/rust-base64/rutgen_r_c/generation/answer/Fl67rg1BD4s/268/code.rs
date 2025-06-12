// Answer 0

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = &[b'A', b'B', b'C', b'=']; // 4 bytes, last byte is padding
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        // Fill decode table as needed
        table
    };
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

    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidPadding)));
}


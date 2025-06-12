// Answer 0

#[test]
fn test_decode_suffix_invalid_length() {
    let input: &[u8] = &[b'A', b'B', b'=', b'='];
    let input_index = 0;
    let mut output: [u8; 2] = [0; 2];
    let output_index = 0;
    let decode_table: [u8; 256] = { 
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
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

    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidLength(
            input_index + 1
        )))
    );
}

#[test]
#[should_panic]
fn test_decode_suffix_panic_out_of_bounds() {
    let input: &[u8] = &[b'A', b'B', b'Z', b'C'];
    let input_index = 0;
    let mut output: [u8; 2] = [0; 2];
    let output_index = 2; // Intentionally set to go out of bounds
    let decode_table: [u8; 256] = { 
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    // This should panic
    let _ = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );
}

#[test]
fn test_decode_suffix_trailing_byte_invalid_length() {
    let input: &[u8] = &[b'A', b'B', b'C'];
    let input_index = 0;
    let mut output: [u8; 2] = [0; 2];
    let output_index = 0;
    let decode_table: [u8; 256] = { 
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
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

    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidLength(
            input_index + 1
        )))
    );
}


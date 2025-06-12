// Answer 0

#[test]
fn test_decode_suffix_with_full_padding() {
    let input: &[u8] = b"AA==";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut tbl = [INVALID_VALUE; 256];
        // Assume 'A' -> 0, 'B' -> 1, etc. for test purposes
        tbl[b'A' as usize] = 0;
        tbl[b'B' as usize] = 1;
        tbl[b'=' as usize] = PAD_BYTE; // Assuming this represents padding
        tbl
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());
    assert_eq!(output[..1], [0]); // Expecting 'A' = 0
}

#[test]
fn test_decode_suffix_with_invalid_padding() {
    let input: &[u8] = b"AAB";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut tbl = [INVALID_VALUE; 256];
        tbl[b'A' as usize] = 0;
        tbl[b'B' as usize] = 1;
        tbl[b'=' as usize] = PAD_BYTE;
        tbl
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_err());
}

#[test]
fn test_decode_suffix_with_no_padding() {
    let input: &[u8] = b"AA";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut tbl = [INVALID_VALUE; 256];
        tbl[b'A' as usize] = 0;
        tbl[b'B' as usize] = 1;
        tbl
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());
    assert_eq!(output[..1], [0]); // Expecting 'A' = 0
}

#[test]
fn test_decode_suffix_with_invalid_trailing_bits() {
    let input: &[u8] = b"AA\xFF";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut tbl = [INVALID_VALUE; 256];
        tbl[b'A' as usize] = 0;
        tbl[b'=' as usize] = PAD_BYTE;
        tbl
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_err());
}


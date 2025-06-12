// Answer 0

#[test]
fn test_decode_suffix_valid_input() {
    let input: &[u8] = &[0b00000000, 0b00000001, 0b00000010, 0b00000011]; // This is a valid input of 4 bytes.
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i + 0x41] = i;  // A-Z
            table[i + 0x61] = i + 26; // a-z
            table[i + 0x30] = i + 52; // 0-9
        }
        table[0x2B] = 62; // +
        table[0x2F] = 63; // /
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
    assert_eq!(metadata.decoded_len, 4);
    assert_eq!(metadata.padding_offset, None);
    assert_eq!(&output, &[0; 4]); // Since input is for decode to 4 bytes, output remains zeros.
}

#[test]
#[should_panic]
fn test_decode_suffix_invalid_byte() {
    let input: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF]; // Invalid input with bytes not in the decode table.
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i + 0x41] = i;  // A-Z
            table[i + 0x61] = i + 26; // a-z
            table[i + 0x30] = i + 52; // 0-9
        }
        table[0x2B] = 62; // +
        table[0x2F] = 63; // /
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    decode_suffix(
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
fn test_decode_suffix_with_trailing_bits() {
    let input: &[u8] = &[0b00000000, 0b00000001]; // Valid base64 representation needing two additional symbols.
    let input_index: usize = 0;
    let mut output: [u8; 2] = [0; 2];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i + 0x41] = i;  // A-Z
            table[i + 0x61] = i + 26; // a-z
            table[i + 0x30] = i + 52; // 0-9
        }
        table[0x2B] = 62; // +
        table[0x2F] = 63; // /
        table
    };
    let decode_allow_trailing_bits = true; // Allow non-canonical bits.
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
    assert_eq!(metadata.decoded_len, 1);
    assert_eq!(metadata.padding_offset, None);
    assert_eq!(&output, &[0, 0]); // We are left with zeroes as input is not completely decoding here.
}


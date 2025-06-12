// Answer 0

#[test]
fn test_decode_suffix_valid_case() {
    let input: &[u8] = &[0b11000000, 0b11000001, 0b10110000, 0b10111000]; // Example valid input
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &byte) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[byte as usize] = i as u8;
        }
        table[b'=' as usize] = PAD_BYTE; // Set the padding byte
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
    assert_eq!(metadata.output_index, 2); // Expected output index after decoding
    assert!(metadata.first_padding_index.is_some()); // There should be a padding index
}

#[test]
#[should_panic] // This should panic due to out of bounds on input slicing
fn test_decode_suffix_panic_on_out_of_bounds() {
    let input: &[u8] = &[0b11000000, 0b11000001, 0b10110000]; // Only 3 bytes
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;

    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &byte) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[byte as usize] = i as u8;
        }
        table[b'=' as usize] = PAD_BYTE; // Set the padding byte
        table
    };

    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

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
fn test_decode_suffix_with_invalid_last_symbol() {
    let input: &[u8] = &[0b11000000, 0b11000001, 0b10110000, 0b11111111]; // Invalid last symbol
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;

    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &byte) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[byte as usize] = i as u8;
        }
        table[b'=' as usize] = PAD_BYTE; // Set the padding byte
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

    assert!(result.is_err()); // Expect an error due to invalid last symbol
}


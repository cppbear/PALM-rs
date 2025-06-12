// Answer 0

#[test]
fn test_decode_suffix_valid_case() {
    const DECODE_TABLE: [u8; 256] = [
        // Fill with dummy values for testing
        INVALID_VALUE; 256
    ];
    
    let input: &[u8] = b"some_base64_input==";
    let input_index = input.len() - 4;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, output_index);
    assert_eq!(metadata.padding_index, None);
}

#[test]
#[should_panic]
fn test_decode_suffix_invalid_padding_first() {
    const DECODE_TABLE: [u8; 256] = [
        // Fill with dummy values for testing
        INVALID_VALUE; 256
    ];

    let input: &[u8] = b"somebase64==";
    let input_index = input.len() - 4;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    ).unwrap();
}

#[test]
fn test_decode_suffix_no_trailing_bits() {
    const DECODE_TABLE: [u8; 256] = [
        // Fill with dummy values for testing
        INVALID_VALUE; 256
    ];

    let input: &[u8] = b"abc="; // Example input
    let input_index = 0; // Start decoding from the beginning
    let mut output = [0u8; 4]; // Prepare output buffer
    let mut output_index = 0; // Output index starts at 0
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, output_index + 1); // Adjusted based on valid decoding
    assert_eq!(metadata.padding_index, None);
}

#[test]
fn test_decode_suffix_invalid_last_symbol() {
    const DECODE_TABLE: [u8; 256] = [
        // Fill with dummy values for testing
        INVALID_VALUE; 256
    ];

    let input: &[u8] = b"xyz$"; // Invalid last symbol
    let input_index = 0; // Start decoding from the beginning
    let mut output = [0u8; 4]; // Prepare output buffer
    let mut output_index = 0; // Output index starts at 0
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
}


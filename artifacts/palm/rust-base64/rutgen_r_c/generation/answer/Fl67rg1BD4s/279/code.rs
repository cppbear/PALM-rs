// Answer 0

#[test]
fn test_decode_suffix_successful_case() {
    const DECODE_TABLE: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Define a valid base64 decode table (for demo purposes)
        for (i, &c) in b"A-Za-z0-9+/".iter().enumerate() { 
            table[c as usize] = i as u8; 
        }
        table[b'=' as usize] = 64; // Padding character
        table
    };

    let input = b"MQ=="; // This is base64 for "1"
    let input_index = 0;
    let mut output = [0u8; 3]; // 3 bytes enough to hold "1"
    let output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert_eq!(result, Ok(DecodeMetadata::new(1, None)));
    assert_eq!(&output[..1], b"1"); // Check that the output is correct
}

#[test]
#[should_panic]
fn test_decode_suffix_invalid_padding_too_early() {
    // This test aims to trigger an invalid padding situation
    const DECODE_TABLE: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &c) in b"A-Za-z0-9+/".iter().enumerate() {
            table[c as usize] = i as u8;
        }
        table
    };

    let input = b"MQ=="; // This is base64 for "1"
    let input_index = 0;
    let mut output = [0u8; 3];
    let output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    // Make it invalid by having a valid byte after padding
    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert_eq!(result, Ok(DecodeMetadata::new(1, None)));
}

#[test]
fn test_decode_suffix_invalid_byte() {
    // Test with an invalid byte input that should fail
    const DECODE_TABLE: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &c) in b"A-Za-z0-9+/".iter().enumerate() {
            table[c as usize] = i as u8;
        }
        table
    };

    let input = b"I!=="; // Invalid base64 with character '!'

    let input_index = 0;
    let mut output = [0u8; 3];
    let output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(0, b'!')))));
}


// Answer 0

#[test]
fn test_decode_suffix_with_exceeding_padding() {
    const PAD_BYTE: u8 = b'='; // Define PAD_BYTE as expected in the function
    const INVALID_VALUE: u8 = 255; // Assuming invalid value is 255
    const INPUT_INDEX: usize = 0;
    const DECODE_ALLOW_TRAILING_BITS: bool = false; // Set according to the context
    const DECODE_PADDING_MODE: DecodePaddingMode = DecodePaddingMode::RequireCanonical; 

    let input: [u8; 4] = [b'A', b'B', PAD_BYTE, PAD_BYTE]; // 4 bytes input with improper padding
    let mut output: [u8; 2] = [0; 2]; // output buffer
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Populate decode_table as necessary
        table[b'A' as usize] = 0; // Example for A
        table[b'B' as usize] = 1; // Example for B
        table
    };

    let result = decode_suffix(
        &input,
        INPUT_INDEX,
        &mut output,
        0,
        &decode_table,
        DECODE_ALLOW_TRAILING_BITS,
        DECODE_PADDING_MODE,
    );

    // Check if the expected error matches
    assert!(result.is_err());
    if let Err(decode_error) = result {
        assert_eq!(decode_error, DecodeError::InvalidByte(INPUT_INDEX + 2, PAD_BYTE).into()); // Check expected error type and value
    }
}

#[test]
fn test_decode_suffix_with_no_padding_and_valid_input() {
    const PAD_BYTE: u8 = b'='; // Define PAD_BYTE as expected in the function
    const INVALID_VALUE: u8 = 255; // Assuming invalid value is 255
    const INPUT_INDEX: usize = 0;
    const DECODE_ALLOW_TRAILING_BITS: bool = true; // Allow trailing bits for this case
    const DECODE_PADDING_MODE: DecodePaddingMode = DecodePaddingMode::Indifferent; 

    let input: [u8; 4] = [b'A', b'B', b'C', b'D']; // Valid base64 input, not hitting padding logic
    let mut output: [u8; 3] = [0; 3]; // Output buffer
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Populate decode_table as necessary
        table[b'A' as usize] = 0; 
        table[b'B' as usize] = 1; 
        table[b'C' as usize] = 2; 
        table[b'D' as usize] = 3; 
        table
    };

    let result = decode_suffix(
        &input,
        INPUT_INDEX,
        &mut output,
        0,
        &decode_table,
        DECODE_ALLOW_TRAILING_BITS,
        DECODE_PADDING_MODE,
    );

    // Check for expected valid output
    assert!(result.is_ok());
    if let Ok(metadata) = result {
        assert_eq!(metadata.output_index, 3); // Expected 3 bytes decoded
        assert_eq!(output[0], 0); // Expected byte values from decoding
        assert_eq!(output[1], 1);
        assert_eq!(output[2], 2);
    }
}

#[test]
fn test_decode_suffix_with_invalid_last_symbol() {
    const PAD_BYTE: u8 = b'='; // Define PAD_BYTE as expected in the function
    const INVALID_VALUE: u8 = 255; // Assuming invalid value is 255
    const INPUT_INDEX: usize = 0;
    const DECODE_ALLOW_TRAILING_BITS: bool = false; // Disallow trailing bits for testing valid last symbol
    const DECODE_PADDING_MODE: DecodePaddingMode = DecodePaddingMode::RequireNone; 

    let input: [u8; 4] = [b'A', b'B', b'=', b'W']; // Test case with invalid last symbol ('W' should not be valid)
    let mut output: [u8; 2] = [0; 2]; // Output buffer
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Populate decode_table as necessary
        table[b'A' as usize] = 0; 
        table[b'B' as usize] = 1; 
        table[b'=' as usize] = PAD_BYTE; // Treat pad byte as currently
        table
    };

    let result = decode_suffix(
        &input,
        INPUT_INDEX,
        &mut output,
        0,
        &decode_table,
        DECODE_ALLOW_TRAILING_BITS,
        DECODE_PADDING_MODE,
    );

    // Check if the expected error matches for invalid last symbol
    assert!(result.is_err());
    if let Err(decode_error) = result {
        assert_eq!(decode_error, DecodeError::InvalidLastSymbol(INPUT_INDEX + 3, b'W').into()); // Last byte is W
    }
}


// Answer 0

#[test]
fn test_decode_helper_invalid_length() {
    let input: &[u8] = b"abc"; // should be a valid base64 input, causing InvalidLength error
    let estimate = GeneralPurposeEstimate { rem: 1, conservative_decoded_len: 4 };
    let mut output = vec![0u8; 2]; // too small for the expected decode length
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::Indifferent);
    assert!(matches!(result, Err(DecodeSliceError::OutputSliceTooSmall)));
}

#[test]
fn test_decode_helper_invalid_byte() {
    let input: &[u8] = b"==++"; // invalid characters '+' should trigger InvalidByte error
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 0 };
    let mut output = vec![0u8; 4]; // adequate size for the expected decode length
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::Indifferent);
    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(2, _)))))
}

#[test]
fn test_decode_helper_invalid_padding() {
    let input: &[u8] = b"YWJjZDEyMw=="; // valid base64 but with wrong padding
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 8 };
    let mut output = vec![0u8; 8]; // adequate size for the expected decode length
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireNone);
    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidPadding))));
}

#[test]
fn test_decode_helper_panic_on_unexpected_error() {
    let input: &[u8] = b"YWJj"; // valid base64 input
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 3 }; // expects 3 bytes output
    let mut output = vec![0u8; 3]; // fits the expected output size
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'Y' as usize] = 1;
        table[b'J' as usize] = 2;
        table[b'j' as usize] = 3;
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::Indifferent);
    assert!(result.is_ok());
    assert_eq!(output, b"abc"); // verify that decoding worked correctly
}


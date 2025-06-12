// Answer 0

#[test]
fn test_decode_helper_valid_input() {
    let input: &[u8] = b"SGVsbG8gd29ybGQ="; // "Hello world" in base64
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 12 };
    let mut output = [0u8; 12];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') as u8;
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a' + 26) as u8;
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0' + 52) as u8;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table[b'=' as usize] = 0; // Not used in decoding, but allow it
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 12);
    assert!(metadata.padding_offset.is_none());
    assert_eq!(&output, b"Hello world");
}

#[test]
#[should_panic]
fn test_decode_helper_invalid_input_too_short() {
    let input: &[u8] = b""; // Empty input
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 0 };
    let mut output = [0u8; 12];
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All invalid for this test
    let _ = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_decode_helper_invalid_character() {
    let input: &[u8] = b"SGVsbG8gd29ybGQ%!"; // Invalid character '%'
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 12 };
    let mut output = [0u8; 12];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') as u8;
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a' + 26) as u8;
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0' + 52) as u8;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table[b'=' as usize] = 0; // Not used in decoding, but allow it
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(_, _)))));
}

#[test]
fn test_decode_helper_with_padding() {
    let input: &[u8] = b"SGVsbG8gd29ybGQ="; // "Hello world" in base64 with padding
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 12 };
    let mut output = [0u8; 12];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') as u8;
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a' + 26) as u8;
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0' + 52) as u8;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table[b'=' as usize] = 0; // Not used in decoding, but allow it
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 12);
    assert!(metadata.padding_offset.is_none());
    assert_eq!(&output, b"Hello world");
}


// Answer 0

#[test]
fn test_decode_helper_success() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    let input: &[u8] = b"QUJD"; // Base64 for ABC
    let mut output: [u8; 3] = [0; 3];
    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 3,
    };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let padding_mode = DecodePaddingMode::Indifferent;

    let metadata = decode_helper(input, &estimate, &mut output, &decode_table, false, padding_mode).unwrap();
    assert_eq!(output, [65, 66, 67]); // Expected output for ABC
    assert_eq!(metadata.decoded_len, 3);
    assert_eq!(metadata.padding_offset, None);
}


#[test]
#[should_panic]
fn test_decode_helper_partial_input() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    let input: &[u8] = b"QUJ"; // Incomplete Base64
    let mut output: [u8; 4] = [0; 4]; // Output buffer is larger to test panic
    let estimate = GeneralPurposeEstimate {
        rem: 1,
        conservative_decoded_len: 3,
    };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let padding_mode = DecodePaddingMode::Indifferent;

    decode_helper(input, &estimate, &mut output, &decode_table, false, padding_mode).unwrap();
}

#[test]
fn test_decode_helper_invalid_padding() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    let input: &[u8] = b"QUJD=="; // Base64 for ABC with invalid padding too many '='
    let mut output: [u8; 3] = [0; 3];
    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 3,
    };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let padding_mode = DecodePaddingMode::RequireNone;

    assert!(decode_helper(input, &estimate, &mut output, &decode_table, false, padding_mode).is_err());
}



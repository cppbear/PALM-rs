// Answer 0

#[test]
fn test_decode_helper_valid_input() {
    struct Estimator {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = Estimator {
        rem: 0,
        conservative_decoded_len: 24,
    };

    let input: &[u8] = b"QUJDRA=="; // Base64 for "ABCD"
    let mut output = vec![0u8; 3]; // Need 3 bytes for "ABCD"

    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' + i as u8 as usize] = i as u8;
        }
        for i in 0..26 {
            table[b'a' + i as u8 as usize] = (i + 26) as u8;
        }
        for i in 0..10 {
            table[b'0' + i as u8 as usize] = (i + 52) as u8;
        }
        table[b'+'] = 62;
        table[b'/'] = 63;
        table[b'='] = PAD_BYTE; // Assuming PAD_BYTE is the same as b'='
        table
    };

    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 3);
}

#[test]
#[should_panic]
fn test_decode_helper_invalid_input_size() {
    // Input is too short for provided output size
    struct Estimator {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = Estimator {
        rem: 0,
        conservative_decoded_len: 10, // More than can be provided by input
    };

    let input: &[u8] = b"QUJDRA=="; // Base64 for "ABCD"
    let mut output = vec![0u8; 5]; // Need 5 bytes but decoding only gives 3

    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Fill decode table (as done in the previous test)
        for i in 0..26 {
            table[b'A' + i as u8 as usize] = i as u8;
        }
        for i in 0..26 {
            table[b'a' + i as u8 as usize] = (i + 26) as u8;
        }
        for i in 0..10 {
            table[b'0' + i as u8 as usize] = (i + 52) as u8;
        }
        table[b'+'] = 62;
        table[b'/'] = 63;
        table[b'='] = PAD_BYTE; // Assuming PAD_BYTE is the same as b'='
        table
    };

    decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical).unwrap();
}

#[test]
fn test_decode_helper_with_padding() {
    struct Estimator {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = Estimator {
        rem: 0,
        conservative_decoded_len: 6,
    };

    let input: &[u8] = b"QUJDRA=="; // Base64 for "ABCD" with padding
    let mut output = vec![0u8; 4]; // Need 4 bytes

    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' + i as u8 as usize] = i as u8;
        }
        for i in 0..26 {
            table[b'a' + i as u8 as usize] = (i + 26) as u8;
        }
        for i in 0..10 {
            table[b'0' + i as u8 as usize] = (i + 52) as u8;
        }
        table[b'+'] = 62;
        table[b'/'] = 63;
        table[b'='] = PAD_BYTE; // Assuming PAD_BYTE is the same as b'='
        table
    };

    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 3);
}

#[test]
#[should_panic]
fn test_decode_helper_invalid_byte() {
    struct Estimator {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = Estimator {
        rem: 0,
        conservative_decoded_len: 3,
    };

    let input: &[u8] = b"QUJD#A=="; // Invalid character '#' in input
    let mut output = vec![0u8; 3];

    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' + i as u8 as usize] = i as u8;
        }
        for i in 0..26 {
            table[b'a' + i as u8 as usize] = (i + 26) as u8;
        }
        for i in 0..10 {
            table[b'0' + i as u8 as usize] = (i + 52) as u8;
        }
        table[b'+'] = 62;
        table[b'/'] = 63;
        table[b'='] = PAD_BYTE;
        table
    };

    decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical).unwrap();
}


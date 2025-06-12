// Answer 0

#[test]
fn test_complete_quads_len_valid_input() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Filling the decode_table for valid Base64 characters (A-Z, a-z, 0-9, +, /)
        for i in 0..26 {
            table[i as usize + b'A' as usize] = i;
            table[i as usize + b'a' as usize] = i + 26;
        }
        for i in 0..10 {
            table[i as usize + b'0' as usize] = i + 52;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };

    assert_eq!(complete_quads_len(b"QUJD", 0, 3, &decode_table), Ok(4));
    assert_eq!(complete_quads_len(b"QUJD=", 1, 3, &decode_table), Ok(4));
    assert_eq!(complete_quads_len(b"QUJD==", 2, 3, &decode_table), Ok(4));
}

#[test]
fn test_complete_quads_len_invalid_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Filling the decode_table for valid Base64 characters (A-Z, a-z, 0-9, +, /)
        for i in 0..26 {
            table[i as usize + b'A' as usize] = i;
            table[i as usize + b'a' as usize] = i + 26;
        }
        for i in 0..10 {
            table[i as usize + b'0' as usize] = i + 52;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };

    assert_eq!(complete_quads_len(b"QUJD\xFF", 1, 3, &decode_table), Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(4, 0xFF))));
}

#[test]
fn test_complete_quads_len_output_slice_too_small() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[i as usize + b'A' as usize] = i;
            table[i as usize + b'a' as usize] = i + 26;
        }
        for i in 0..10 {
            table[i as usize + b'0' as usize] = i + 52;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };

    assert_eq!(complete_quads_len(b"QUJD", 0, 2, &decode_table), Err(DecodeSliceError::OutputSliceTooSmall));
}


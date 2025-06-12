// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

const INVALID_VALUE: u8 = 255;

#[test]
fn test_decode_chunk_4_valid_input() {
    let decode_table: [u8; 256] = [
        // Fill with valid base64 decode values for testing
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 0-15
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, // 16-31
        INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, // 32-35
        // ... Other valid values up to 255
        // Assume 'A' -> 0, 'B' -> 1, ..., 'O' -> 14
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, // Fill rest as needed
    ];
    
    let input: &[u8] = &[b'A', b'B', b'C', b'D'];
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, [0, 1, 2]); // Expected output based on values in decode_table
}

#[test]
fn test_decode_chunk_4_invalid_byte_at_index_2() {
    let decode_table: [u8; 256] = [
        // Fill with valid base64 decode values for testing
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 0-15
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, // 16-31
        INVALID_VALUE, // At least one invalid entry for index 2
        // ... Other valid values up to 255
    ];
    
    let input: &[u8] = &[b'A', b'B', b'!', b'D']; // '!' is invalid
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError { index, byte }) = result {
        assert_eq!(index, 2);
        assert_eq!(byte, b'!');
    }
}


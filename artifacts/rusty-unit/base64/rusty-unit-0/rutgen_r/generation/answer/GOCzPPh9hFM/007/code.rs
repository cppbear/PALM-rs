// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // valid base64 value for 'A'
        table[b'B' as usize] = 1; // valid base64 value for 'B'
        table[b'C' as usize] = 2; // valid base64 value for 'C'
        table[b'D' as usize] = 3; // valid base64 value for 'D'
        table[b'E' as usize] = 4; // valid base64 value for 'E'
        table[b'F' as usize] = 5; // valid base64 value for 'F'
        table[b'G' as usize] = 6; // valid base64 value for 'G'
        table[b'H' as usize] = 7; // valid base64 value for 'H'
        table
    };

    let input: &[u8] = b"ABCDEFGH"; // valid input to decode
    let expected_output: [u8; 6] = [0, 1, 2, 3, 4, 5]; // expected output after decoding
    let mut output = [0u8; 6];
    
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, expected_output);
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // valid base64 value for 'A'
        table[b'B' as usize] = 1; // valid base64 value for 'B'
        table[b'C' as usize] = 2; // valid base64 value for 'C'
        table[b'D' as usize] = 3; // valid base64 value for 'D'
        table[b'E' as usize] = 4; // valid base64 value for 'E'
        table[b'F' as usize] = 5; // valid base64 value for 'F'
        table[b'G' as usize] = 6; // valid base64 value for 'G'
        table[b'H' as usize] = 7; // valid base64 value for 'H'
        // no valid value for 'I', keeping it INVALID_VALUE
        table
    };

    let input: &[u8] = b"ABCDEFGI"; // 'I' is invalid
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 6);
        assert_eq!(byte, b'I');
    } else {
        panic!("Expected DecodeError::InvalidByte");
    }
}


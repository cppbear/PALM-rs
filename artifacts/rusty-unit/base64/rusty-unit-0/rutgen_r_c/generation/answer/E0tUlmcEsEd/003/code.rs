// Answer 0

#[test]
fn test_decode_chunk_4_valid_input() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Assuming standard Base64 index values for 'A' to 'Z', 'a' to 'z', '0' to '9', '+', '/'
        for i in 0..26 {
            table[b'A' + i] = i;
        }
        for i in 0..26 {
            table[b'a' + i] = 26 + i;
        }
        for i in 0..10 {
            table[b'0' + i] = 52 + i;
        }
        table[b'+'] = 62;
        table[b'/'] = 63;
        table
    };

    let input: &[u8] = b"QUJD"; // Base64 for "ABC"
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert_eq!(result, Ok(()));
    assert_eq!(output, b"ABC");
}

#[test]
fn test_decode_chunk_4_invalid_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' + i] = i;
        }
        for i in 0..26 {
            table[b'a' + i] = 26 + i;
        }
        for i in 0..10 {
            table[b'0' + i] = 52 + i;
        }
        table[b'+'] = 62;
        table[b'/'] = 63;
        table
    };

    let input: &[u8] = b"QU*C"; // Invalid character '*'
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(2, b'*')));
}


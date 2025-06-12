// Answer 0

#[test]
fn test_decode_chunk_8_success() {
    let decode_table: [u8; 256] = [
        // Initialize the decode table with valid values for characters 'A' to 'Z', 'a' to 'z', '0' to '9', and '+' and '/'
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, // ' ' to '9'
        62, // '+' 
        63, // '/'
        // Initialize the remaining indices with INVALID_VALUE
        [INVALID_VALUE; 256 - 64][..].try_into().unwrap()
    ];
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H'];
    let index_at_start_of_input = 0;
    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    assert!(result.is_ok());
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let decode_table: [u8; 256] = [
        // Initialize the decode table with valid values for characters 'A' to 'Z'
        // and values for 'a' to 'z', '0' to '9', and '+' and '/'
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, // ' ' to '9'
        62, // '+'
        63, // '/'
        // Initialize the remaining indices with INVALID_VALUE
        [INVALID_VALUE; 256 - 64][..].try_into().unwrap()
    ];
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'Z', b'H']; // 'Z' is invalid
    let index_at_start_of_input = 0;
    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 6);
        assert_eq!(byte, b'Z');
    }
}


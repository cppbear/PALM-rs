// Answer 0

#[test]
fn test_decode_chunk_4_valid_input() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; // A -> 0
    decode_table[b'B' as usize] = 1; // B -> 1
    decode_table[b'C' as usize] = 2; // C -> 2
    decode_table[b'D' as usize] = 3; // D -> 3
    // Fill in more valid mappings as needed

    let input: &[u8] = b"ABCD";
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);

    assert!(result.is_ok());
    assert_eq!(output, [0, 1, 2]); // Assuming ABCD translates to 0, 1, 2
}

#[test]
fn test_decode_chunk_4_invalid_byte() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; 
    decode_table[b'B' as usize] = 1; 

    let input: &[u8] = b"ABX"; // X is invalid
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);

    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(offset, byte)) = result {
        assert_eq!(offset, 2);
        assert_eq!(byte, b'X');
    } else {
        panic!("Expected InvalidByte error");
    }
}

#[test]
fn test_decode_chunk_4_exceeding_length() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; 
    decode_table[b'B' as usize] = 1; 
    decode_table[b'C' as usize] = 2; 
    decode_table[b'D' as usize] = 3;

    let input: &[u8] = b"AB"; // Only 2 bytes are provided
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    
    assert!(result.is_err());
    if let Err(DecodeError::InvalidLength(length)) = result {
        assert_eq!(length, 2);
    } else {
        panic!("Expected InvalidLength error");
    }
}


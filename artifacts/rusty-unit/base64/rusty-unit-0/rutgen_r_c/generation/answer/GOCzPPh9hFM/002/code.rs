// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Fill in a sample decode table for Base64 (assuming some values)
        for (i, c) in b"A-Za-z0-9+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table[b'='] = INVALID_VALUE; // Padding character not used for decoding
        table
    };
    
    let input = b"QUJDREU=";  // Represents "ABCDE" in base64, to be decoded
    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input[..8], 0, &decode_table, &mut output);
    
    assert!(result.is_ok());
    assert_eq!(&output[..6], b"ABCDE");
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"A-Za-z0-9+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table
    };

    let input = b"QUJDRE*";  // '*' is an invalid byte for base64
    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input[..8], 0, &decode_table, &mut output);
    
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 6);
        assert_eq!(byte, b'*');
    } else {
        panic!("Expected invalid byte error");
    }
} 

#[test]
fn test_decode_chunk_8_invalid_length() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"A-Za-z0-9+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table
    };

    let input = b"QUJDRE";  // Length of input is less than required for full decode
    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input[..6], 0, &decode_table, &mut output);
    
    assert!(result.is_err());
    if let Err(DecodeError::InvalidLength(len)) = result {
        assert_eq!(len, 6);
    } else {
        panic!("Expected invalid length error");
    }
}


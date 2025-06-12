// Answer 0

#[test]
fn test_decode_chunk_4_valid_input() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b"Y"[0] as usize] = 24; // Y -> 24
        table[b"V"[0] as usize] = 21; // V -> 21
        table[b"F"[0] as usize] = 5;  // F -> 5
        table[b"AY"[0] as usize] = 0;  // A -> 0
        table
    };
    let input: &[u8] = b"YVF";
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, [0x19, 0x15, 0x00]); // Expected valid output based on decode values
}

#[test]
fn test_decode_chunk_4_invalid_first_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b"Y"[0] as usize] = 24; // Y -> 24
        table[b"V"[0] as usize] = 21; // V -> 21
        // F is invalid
        table[b"F"[0] as usize] = INVALID_VALUE; // F -> INVALID
        table
    };
    let input: &[u8] = b"YFZ"; // Input with invalid second byte
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeError::InvalidByte(1, b'F'))); // Assert that error is from index 1
}

#[test]
fn test_decode_chunk_4_invalid_second_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b"Y"[0] as usize] = 24; // Y -> 24
        // V is invalid
        table[b"V"[0] as usize] = INVALID_VALUE; // V -> INVALID
        table[b"F"[0] as usize] = 5; // F -> 5
        table
    };
    let input: &[u8] = b"YVF"; // Input with invalid first byte
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeError::InvalidByte(1, b'V'))); // Assert that error is from index 1
}

#[test]
fn test_decode_chunk_4_invalid_third_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b"Y"[0] as usize] = 24; // Y -> 24
        table[b"V"[0] as usize] = 21; // V -> 21
        // D is invalid
        table[b"D"[0] as usize] = INVALID_VALUE; // D -> INVALID
        table
    };
    let input: &[u8] = b"YVD"; // Input with invalid third byte
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeError::InvalidByte(2, b'D'))); // Assert that error is from index 2
}

#[test]
fn test_decode_chunk_4_invalid_fourth_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b"Y"[0] as usize] = 24; // Y -> 24
        table[b"V"[0] as usize] = 21; // V -> 21
        table[b"F"[0] as usize] = 5;  // F -> 5
        // Z is invalid
        table[b"Z"[0] as usize] = INVALID_VALUE; // Z -> INVALID
        table
    };
    let input: &[u8] = b"YVFZ"; // Input with invalid fourth byte
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeError::InvalidByte(3, b'Z'))); // Assert that error is from index 3
}


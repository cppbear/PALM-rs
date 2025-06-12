// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H'];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table[b'H' as usize] = 7;
        table
    };
    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input, 0, &decode_table, &mut output);
    assert!(result.is_ok());
}

#[test]
fn test_decode_chunk_8_invalid_byte_at_position_5() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'X', b'G', b'H']; // 'X' is invalid
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        // 'X' has no valid value
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table[b'H' as usize] = 7;
        table
    };
    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(pos, byte)) = result {
        assert_eq!(pos, 5);
        assert_eq!(byte, b'X');
    } else {
        panic!("Expected InvalidByte error");
    }
}

#[test]
fn test_decode_chunk_8_invalid_byte_at_position_6() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'X', b'H']; // 'X' is invalid
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        // 'X' has no valid value
        table[b'G' as usize] = 6;
        table[b'H' as usize] = 7;
        table
    };
    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(pos, byte)) = result {
        assert_eq!(pos, 6);
        assert_eq!(byte, b'X');
    } else {
        panic!("Expected InvalidByte error");
    }
}


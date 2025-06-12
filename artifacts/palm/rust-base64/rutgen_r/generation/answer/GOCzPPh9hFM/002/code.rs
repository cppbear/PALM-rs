// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

const INVALID_VALUE: u8 = 255;

#[test]
fn test_valid_input() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;
    decode_table[b'C' as usize] = 2;
    decode_table[b'D' as usize] = 3;
    decode_table[b'E' as usize] = 4;
    decode_table[b'F' as usize] = 5;
    decode_table[b'G' as usize] = 6;
    decode_table[b'H' as usize] = 7;
    decode_table[b'I' as usize] = 8;
    decode_table[b'J' as usize] = 9;
    decode_table[b'K' as usize] = 10;
    decode_table[b'L' as usize] = 11;
    decode_table[b'M' as usize] = 12;
    decode_table[b'N' as usize] = 13;
    decode_table[b'O' as usize] = 14;
    decode_table[b'P' as usize] = 15;
    decode_table[b'Q' as usize] = 16;
    decode_table[b'R' as usize] = 17;
    decode_table[b'S' as usize] = 18;
    decode_table[b'T' as usize] = 19;
    decode_table[b'U' as usize] = 20;
    decode_table[b'V' as usize] = 21;
    decode_table[b'W' as usize] = 22;
    decode_table[b'X' as usize] = 23;
    decode_table[b'Y' as usize] = 24;
    decode_table[b'Z' as usize] = 25;

    let input: &[u8] = b"ABCDEFGH";
    let mut output = [0u8; 6];
    assert!(decode_chunk_8(input, 0, &decode_table, &mut output).is_ok());
}

#[test]
fn test_invalid_first_byte() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    
    let input: &[u8] = b"\xFFBCDEFGH";
    let mut output = [0u8; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError { index, byte }) = result {
        assert_eq!(index, 0);
        assert_eq!(byte, 0xFF);
    }
}

#[test]
fn test_invalid_second_byte() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;
    
    let input: &[u8] = b"AB\xFFDEFGH";
    let mut output = [0u8; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError { index, byte }) = result {
        assert_eq!(index, 2);
        assert_eq!(byte, 0xFF);
    }
}

#[test]
fn test_invalid_third_byte() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;
    decode_table[b'C' as usize] = 2;

    let input: &[u8] = b"ABC\xFFEFGH";
    let mut output = [0u8; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError { index, byte }) = result {
        assert_eq!(index, 3);
        assert_eq!(byte, 0xFF);
    }
}


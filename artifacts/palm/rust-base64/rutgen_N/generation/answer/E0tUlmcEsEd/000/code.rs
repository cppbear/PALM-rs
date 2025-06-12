// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

const INVALID_VALUE: u8 = 255;

#[test]
fn test_decode_chunk_4_valid_input() {
    let input: &[u8] = &[0b00111100, 0b00011001, 0b00001101, 0b00110011]; // some valid input bytes
    let index_start: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00111100] = 0; // example mapping
        table[0b00011001] = 1; // example mapping
        table[0b00001101] = 2; // example mapping
        table[0b00110011] = 3; // example mapping
        table
    };
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, index_start, &decode_table, &mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, [0, 1, 2]); // expected output for given inputs
}

#[test]
fn test_decode_chunk_4_invalid_first_byte() {
    let input: &[u8] = &[0b11111111, 0b00011001, 0b00001101, 0b00110011]; // first byte invalid
    let index_start: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00011001] = 1; // example mapping
        table[0b00001101] = 2; // example mapping
        table[0b00110011] = 3; // example mapping
        table
    };
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, index_start, &decode_table, &mut output);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeError { index: index_start, byte: 0b11111111 });
}

#[test]
fn test_decode_chunk_4_invalid_second_byte() {
    let input: &[u8] = &[0b00111100, 0b11111111, 0b00001101, 0b00110011]; // second byte invalid
    let index_start: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00111100] = 0; // example mapping
        table[0b00001101] = 2; // example mapping
        table[0b00110011] = 3; // example mapping
        table
    };
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, index_start, &decode_table, &mut output);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeError { index: index_start + 1, byte: 0b11111111 });
}

#[test]
fn test_decode_chunk_4_invalid_third_byte() {
    let input: &[u8] = &[0b00111100, 0b00011001, 0b11111111, 0b00110011]; // third byte invalid
    let index_start: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00111100] = 0; // example mapping
        table[0b00011001] = 1; // example mapping
        table[0b00110011] = 3; // example mapping
        table
    };
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, index_start, &decode_table, &mut output);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeError { index: index_start + 2, byte: 0b11111111 });
}

#[test]
fn test_decode_chunk_4_invalid_fourth_byte() {
    let input: &[u8] = &[0b00111100, 0b00011001, 0b00001101, 0b11111111]; // fourth byte invalid
    let index_start: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00111100] = 0; // example mapping
        table[0b00011001] = 1; // example mapping
        table[0b00001101] = 2; // example mapping
        table
    };
    let mut output = [0u8; 3];
    
    let result = decode_chunk_4(input, index_start, &decode_table, &mut output);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeError { index: index_start + 3, byte: 0b11111111 });
}


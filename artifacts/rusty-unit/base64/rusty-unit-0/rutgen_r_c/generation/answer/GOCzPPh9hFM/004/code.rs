// Answer 0

#[test]
fn test_decode_chunk_8_success() {
    let input: [u8; 8] = [0b000000, 0b000001, 0b000010, 0b000011, 0b000100, 0b000101, 0b000110, 0b000111]; // valid input symbols
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i + 65] = i as u8; // A-Z, a-z, 0-9, +, /
        }
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_ok());
    assert_eq!(&output, &[0b00000001, 0b00000010, 0b00000011, 0b00000100, 0b00000101, 0b00000110]); // expected output
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input: [u8; 8] = [0b000000, 0b000001, 0b000010, 0b111111, 0b000100, 0b000101, 0b000110, 0b000111]; // invalid 4th symbol
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i + 65] = i as u8; // A-Z, a-z, 0-9, +, /
        }
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(offset, byte)) = result {
        assert_eq!(offset, index_at_start_of_input + 3);
        assert_eq!(byte, input[3]);
    } else {
        panic!("Unexpected result type");
    }
}


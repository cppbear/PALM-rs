// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let input: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let index_at_start_of_input: usize = 0;

    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for i in 0..8 {
        decode_table[i] = i;
    }

    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Ok(()));
    assert_eq!(output, [0, 0, 0, 0, 0, 0]); // Expected output is system dependent, placeholder value
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let index_at_start_of_input: usize = 0;

    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[0] = 0; // Valid byte
    decode_table[1] = 1; // Valid byte
    decode_table[2] = INVALID_VALUE; // Invalid byte

    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(idx, byte)) = result {
        assert_eq!(idx, index_at_start_of_input + 2);
        assert_eq!(byte, 2);
    }
}

#[test]
fn test_decode_chunk_8_partial_valid() {
    let input: [u8; 8] = [6, 7, 1, 2, 3, 4, 5, 6];
    let index_at_start_of_input: usize = 0;

    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for i in 0..6 {
        decode_table[i] = i;
    }
    decode_table[6] = 6; // Valid byte
    decode_table[7] = 7; // Valid byte

    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(idx, byte)) = result {
        assert_eq!(idx, index_at_start_of_input + 2);
        assert_eq!(byte, 1);
    }
}

#[test]
#[should_panic]
fn test_decode_chunk_8_panic_on_output_slice() {
    let input: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let index_at_start_of_input: usize = 0;

    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for i in 0..8 {
        decode_table[i] = i;
    }

    let mut output: [u8; 4] = [0; 4]; // Intentionally incorrect size

    let _ = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_empty_output() {
    let input: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let index_at_start_of_input: usize = 0;

    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for i in 0..8 {
        decode_table[i] = i;
    }

    let mut output: [u8; 0] = []; // Empty output

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err()); // Expect error due to empty output
}


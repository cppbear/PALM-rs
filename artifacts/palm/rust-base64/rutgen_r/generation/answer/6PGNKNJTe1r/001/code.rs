// Answer 0

#[test]
fn test_complete_quads_len_invalid_last_byte() {
    let input: &[u8] = b"VGhpcyBpcyBhIHRlc3Q";
    let input_len_rem: usize = 2; // input.len() % 4 == 2
    let output_len: usize = 10; // large enough for 2 complete quads
    let decode_table: &[u8; 256] = &[INVALID_VALUE; 256]; // Assume ALL are invalid
    let last_byte = input[input.len() - 1]; // last_byte = 'Q'

    // Set a specific value in the decode_table for last_byte that triggers error
    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[usize::from(last_byte)] = INVALID_VALUE; // Mimics invalid last byte
    
    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    
    assert!(result.is_err());
    if let Err(DecodeSliceError::InvalidByte(pos, byte)) = result {
        assert_eq!(pos, input.len() - 1);
        assert_eq!(byte, last_byte);
    } else {
        panic!("Expected DecodeSliceError::InvalidByte");
    }
}


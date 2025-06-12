// Answer 0

#[test]
fn test_complete_quads_len_valid_case() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Initialize with all as invalid
    decode_table[b'A' as usize] = 0; // Mark valid for character 'A'
    decode_table[b'B' as usize] = 1; // Mark valid for character 'B'
    
    let input: &[u8] = b"QUJD"; // Represents 'A', 'B', 'C' in base64
    let input_len_rem: usize = input.len() % 4; // Should be 0
    let output_len: usize = (input.len() / 4) * 3; // Should be 3

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    
    assert_eq!(result, Ok(input.len() - input_len_rem));
}

#[test]
fn test_complete_quads_len_large_input() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;

    let input: &[u8] = b"QUJDQUJDQUJDQUJD"; // Multiple complete quads
    let input_len_rem: usize = input.len() % 4; // Should be 0
    let output_len: usize = (input.len() / 4) * 3; // Should be 12

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    
    assert_eq!(result, Ok(input.len() - input_len_rem));
}

#[test]
fn test_complete_quads_len_not_enough_output_space() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;

    let input: &[u8] = b"QUJDQUJD"; // Two complete quads
    let input_len_rem: usize = input.len() % 4; // Should be 0
    let output_len: usize = (input.len() / 4) * 3 - 1; // Should be 11

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}


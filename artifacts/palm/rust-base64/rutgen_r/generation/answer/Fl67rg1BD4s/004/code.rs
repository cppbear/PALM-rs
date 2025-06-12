// Answer 0

#[test]
fn test_decode_suffix_with_four_bytes_and_no_padding() {
    let input: &[u8] = &[b'A', b'B', b'C', b'D'];
    let mut output = [0_u8; 3];
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Initialize with invalid values
    decode_table[b'A' as usize] = 0; 
    decode_table[b'B' as usize] = 1; 
    decode_table[b'C' as usize] = 2; 
    decode_table[b'D' as usize] = 3; 

    let input_index = 0;
    let output_index = 0;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, padding_mode);
    
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 3);  // Expecting 3 bytes decoded
    assert_eq!(output, [0, 1, 2]); // Test output based on your decoding logic
}

#[test]
#[should_panic(expected = "InvalidByte")]
fn test_decode_suffix_with_invalid_byte_and_padding() {
    let input: &[u8] = &[b'A', b'B', b'=', b'!'];
    let mut output = [0_u8; 3];
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0; 
    decode_table[b'B' as usize] = 1; 

    let input_index = 0;
    let output_index = 0;
    let padding_mode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, padding_mode);
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn test_decode_suffix_with_single_valid_byte() {
    let input: &[u8] = &[b'A'];
    let mut output = [0_u8; 3];
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;

    let input_index = 0;
    let output_index = 0;
    let padding_mode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, padding_mode);
}

#[test]
fn test_decode_suffix_with_padding_bytes() {
    let input: &[u8] = &[b'A', b'B', b'C', b'='];
    let mut output = [0_u8; 3];
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;
    decode_table[b'C' as usize] = 2;

    let input_index = 0;
    let output_index = 0;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, padding_mode);
    
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 3);
    assert_eq!(output, [0, 1, 2]);
}

#[test]
fn test_decode_suffix_with_multiple_padding_bytes() {
    let input: &[u8] = &[b'A', b'B', b'=', b'='];
    let mut output = [0_u8; 3];
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;

    let input_index = 0;
    let output_index = 0;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, false, padding_mode);
    
    assert!(result.is_err());
}


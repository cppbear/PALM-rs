// Answer 0

#[test]
fn test_decode_suffix_valid_cases() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    // initialize decode_table with valid base64 decoding values here
    // e.g., for 'A', 'B', 'C'...
    // This is a simplified example; you need to fill the decode_table correctly based on actual base64 values.

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

    let input = b"ABCD";
    let output_size = 4; 
    let mut output = vec![0u8; output_size];
    let result = decode_suffix(input, 0, &mut output, 0, &decode_table, true, DecodePaddingMode::Indifferent);
    
    assert_eq!(result.is_ok(), true);
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 4);
    assert!(metadata.padding_offset.is_none());
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn test_decode_suffix_invalid_length() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    // initialize decode_table here the same way as in the previous test.

    let input = b"A";
    let output_size = 4; 
    let mut output = vec![0u8; output_size];
    let _ = decode_suffix(input, 0, &mut output, 0, &decode_table, true, DecodePaddingMode::Indifferent).unwrap();
}

#[test]
#[should_panic(expected = "InvalidByte")]
fn test_decode_suffix_invalid_byte() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    // initialize decode_table here the same way as in the previous tests.

    let input = b"ABCDX"; // X is an invalid byte
    let output_size = 4; 
    let mut output = vec![0u8; output_size];
    let _ = decode_suffix(input, 0, &mut output, 0, &decode_table, true, DecodePaddingMode::Indifferent).unwrap();
}

#[test]
#[should_panic(expected = "InvalidPadding")]
fn test_decode_suffix_invalid_padding() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    // initialize decode_table here the same way as in the previous tests.

    let input = b"ABCD=="; // invalid padding with two padding chars in a full input
    let output_size = 4; 
    let mut output = vec![0u8; output_size];
    let _ = decode_suffix(input, 0, &mut output, 0, &decode_table, true, DecodePaddingMode::RequireCanonical).unwrap();
}


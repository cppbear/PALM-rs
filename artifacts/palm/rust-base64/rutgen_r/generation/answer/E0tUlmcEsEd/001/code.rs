// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_first() {
    let input: &[u8] = &[255, b'A', b'A', b'A']; // 255 is out of range for valid base64 characters
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') + 52; // A-Z -> 26-51
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a') + 26; // a-z -> 0-25
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0') + 52; // 0-9 -> 52-61
        }
        table[b'+' as usize] = 62;  // + -> 62
        table[b'/' as usize] = 63;  // / -> 63
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[0])));
}

#[test]
fn test_decode_chunk_4_invalid_byte_second() {
    let input: &[u8] = &[b'A', 255, b'A', b'A']; // 255 is invalid
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') + 52;
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a') + 26;
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0') + 52;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1])));
}

#[test]
fn test_decode_chunk_4_invalid_byte_third() {
    let input: &[u8] = &[b'A', b'A', 255, b'A']; // 255 is invalid
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') + 52;
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a') + 26;
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0') + 52;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2])));
}

#[test]
fn test_decode_chunk_4_invalid_byte_fourth() {
    let input: &[u8] = &[b'A', b'A', b'A', 255]; // 255 is invalid
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in b'A'..=b'Z' {
            table[i as usize] = (i - b'A') + 52;
        }
        for i in b'a'..=b'z' {
            table[i as usize] = (i - b'a') + 26;
        }
        for i in b'0'..=b'9' {
            table[i as usize] = (i - b'0') + 52;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3])));
}


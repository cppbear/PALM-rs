// Answer 0

#[test]
fn test_decode_chunk_8_valid_input() {
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in 0..64 {
            table[i + b'A' as usize] = i as u8; // A-Z
        }
        for i in 0..26 {
            table[i + b'a' as usize + 26] = (i + 26) as u8; // a-z
        }
        for i in 0..10 {
            table[i + b'0' as usize + 52] = (i + 52) as u8; // 0-9
        }
        table[b'+' as usize] = 62; // +
        table[b'/' as usize] = 63; // /
        table
    };
    
    let input: &[u8] = b"ABCDEFGH";
    let index_at_start_of_input = 0;
    let mut output = [0u8; 6];
    
    assert_eq!(decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output), Ok(()));
    assert_eq!(&output, b"\xF0\x00\x00\x00\x00\x00");
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in 0..64 {
            table[i + b'A' as usize] = i as u8; // A-Z
        }
        for i in 0..26 {
            table[i + b'a' as usize + 26] = (i + 26) as u8; // a-z
        }
        for i in 0..10 {
            table[i + b'0' as usize + 52] = (i + 52) as u8; // 0-9
        }
        table[b'+' as usize] = 62; // +
        table[b'/' as usize] = 63; // /
        table
    };
    
    let input: &[u8] = b"ABCD$EFG";
    let index_at_start_of_input = 4; // the index of the invalid byte ($)
    let mut output = [0u8; 6];
    
    assert_eq!(decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output),
               Err(DecodeError::InvalidByte(index_at_start_of_input, input[4])));
}


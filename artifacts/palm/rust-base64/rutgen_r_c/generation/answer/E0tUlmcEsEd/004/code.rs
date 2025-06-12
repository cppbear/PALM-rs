// Answer 0

#[test]
fn test_decode_chunk_4_invalid_last_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        for i in 0..=25 {
            table[i + b'A' as usize] = i as u8; // A-Z
        }
        for i in 0..=25 {
            table[i + b'a' as usize] = (i + 26) as u8; // a-z
        }
        for i in 0..=9 {
            table[i + b'0' as usize] = (i + 52) as u8; // 0-9
        }
        table[b'+' as usize] = 62; // +
        table[b'/' as usize] = 63; // /
        table
    };

    let input: &[u8] = &[b'A', b'B', b'C', b'X']; // X is not a valid Base64 character
    let index_at_start_of_input = 0;
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    
    match result {
        Err(DecodeError::InvalidByte(offset, byte)) => {
            assert_eq!(offset, index_at_start_of_input + 3);
            assert_eq!(byte, b'X');
        },
        _ => panic!("Expected an InvalidByte error for input byte 'X'"),
    }
}


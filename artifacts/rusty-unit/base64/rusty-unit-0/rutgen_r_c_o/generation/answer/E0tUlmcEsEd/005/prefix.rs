// Answer 0

#[test]
fn test_decode_chunk_4_valid_input() {
    let input: &[u8] = &[0, 1, 2, 3];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i] = i as u8;
        }
        table
    };
    let mut output = [0u8; 3];
    
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_another_valid_input() {
    let input: &[u8] = &[62, 63, 62, 63];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table
    };
    let mut output = [0u8; 3];
    
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_another_case() {
    let input: &[u8] = &[10, 11, 12, 13];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..64 {
            table[i] = i as u8;
        }
        table
    };
    let mut output = [0u8; 3];
    
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}


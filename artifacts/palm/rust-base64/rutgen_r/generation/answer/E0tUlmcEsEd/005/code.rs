// Answer 0

#[test]
fn test_decode_chunk_4_valid_input() {
    let input = [b'A', b'B', b'C', b'D'];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [255; 256];
        table[b'A' as usize] = 0; // Decode 'A'
        table[b'B' as usize] = 1; // Decode 'B'
        table[b'C' as usize] = 2; // Decode 'C'
        table[b'D' as usize] = 3; // Decode 'D'
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Ok(()));
    assert_eq!(output, [0u8, 1u8, 2u8]); // Expecting the decoded output to match
}

#[test]
#[should_panic]
fn test_decode_chunk_4_invalid_first_byte() {
    let input = [b'$', b'B', b'C', b'D']; // '$' is not valid in our decode table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [255; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let mut output = [0u8; 3];

    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
#[should_panic]
fn test_decode_chunk_4_invalid_second_byte() {
    let input = [b'A', b'$', b'C', b'D']; // '$' is not valid in our decode table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [255; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let mut output = [0u8; 3];

    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
#[should_panic]
fn test_decode_chunk_4_invalid_third_byte() {
    let input = [b'A', b'B', b'$', b'D']; // '$' is not valid in our decode table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [255; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let mut output = [0u8; 3];

    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
#[should_panic]
fn test_decode_chunk_4_invalid_fourth_byte() {
    let input = [b'A', b'B', b'C', b'$']; // '$' is not valid in our decode table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [255; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table
    };
    let mut output = [0u8; 3];

    let _ = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);
}


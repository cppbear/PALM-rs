// Answer 0

#[test]
fn test_complete_quads_len_case_1() {
    let input: Vec<u8> = vec![b'A', b'B', b'C', b'D'];
    let input_len_rem = 0;
    let output_len = 3;
    let decode_table: [u8; 256] = (0..=255).map(|x| if x < 64 { x as u8 } else { INVALID_VALUE }).collect::<Vec<u8>>().try_into().unwrap();
    complete_quads_len(&input, input_len_rem, output_len, &decode_table);
}

#[test]
fn test_complete_quads_len_case_2() {
    let input: Vec<u8> = vec![b'E', b'F', b'G', b'H', b'I', b'J', b'K'];
    let input_len_rem = 0;
    let output_len = 6;
    let decode_table: [u8; 256] = (0..=255).map(|x| if x < 64 { x as u8 } else { INVALID_VALUE }).collect::<Vec<u8>>().try_into().unwrap();
    complete_quads_len(&input, input_len_rem, output_len, &decode_table);
}

#[test]
fn test_complete_quads_len_case_3() {
    let input: Vec<u8> = vec![b'M', b'N', b'O', b'P', b'Q', b'R', b'S'];
    let input_len_rem = 0;
    let output_len = 6;
    let decode_table: [u8; 256] = (0..=255).map(|x| if x < 64 { x as u8 } else { INVALID_VALUE }).collect::<Vec<u8>>().try_into().unwrap();
    complete_quads_len(&input, input_len_rem, output_len, &decode_table);
}


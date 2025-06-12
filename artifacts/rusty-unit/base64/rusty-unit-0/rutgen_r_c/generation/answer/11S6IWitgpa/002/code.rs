// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let unpadded_output_len = 0;
    let mut output = [0u8; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 0);
    assert_eq!(output[0], PAD_BYTE);
    assert_eq!(output[1], 0);
}

#[test]
fn test_add_padding_three_bytes_needed() {
    let unpadded_output_len = 1;
    let mut output = [0u8; 3];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 3);
    assert_eq!(output[0], PAD_BYTE);
    assert_eq!(output[1], PAD_BYTE);
    assert_eq!(output[2], PAD_BYTE);
}

#[test]
fn test_add_padding_two_bytes_needed() {
    let unpadded_output_len = 2;
    let mut output = [0u8; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output[0], PAD_BYTE);
    assert_eq!(output[1], PAD_BYTE);
} 

#[test]
fn test_add_padding_one_byte_needed() {
    let unpadded_output_len = 3;
    let mut output = [0u8; 1];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 1);
    assert_eq!(output[0], PAD_BYTE);
}

#[test]
#[should_panic]
fn test_add_padding_output_too_small() {
    let unpadded_output_len = 4;
    let mut output = [0u8; 1];
    add_padding(unpadded_output_len, &mut output);
}


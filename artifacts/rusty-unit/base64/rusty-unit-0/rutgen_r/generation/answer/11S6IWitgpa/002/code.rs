// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let unpadded_output_len = 0; // 0 % 4 == 0, so no padding is needed
    let mut output = [0u8; 2];
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 0);
}

#[test]
fn test_add_padding_one_byte_padding() {
    let unpadded_output_len = 1; // 1 % 4 == 1, so 3 padding bytes needed
    let mut output = [0u8; 4];
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 3);
    assert_eq!(output[0], PAD_BYTE);
    assert_eq!(output[1], PAD_BYTE);
    assert_eq!(output[2], PAD_BYTE);
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let unpadded_output_len = 2; // 2 % 4 == 2, so 2 padding bytes needed
    let mut output = [0u8; 4];
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 2);
    assert_eq!(output[0], PAD_BYTE);
    assert_eq!(output[1], PAD_BYTE);
}

#[test]
fn test_add_padding_three_bytes_padding() {
    let unpadded_output_len = 3; // 3 % 4 == 3, so 1 padding byte needed
    let mut output = [0u8; 4];
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 1);
    assert_eq!(output[0], PAD_BYTE);
}

#[test]
fn test_add_padding_maximum_capacity() {
    let unpadded_output_len = 4; // 4 % 4 == 0, so no padding is needed
    let mut output = [0u8; 4];
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 0);
}


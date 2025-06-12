// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let unpadded_output_len = 0;
    let mut output = [0; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 0);
    assert_eq!(output, [0, 0]);
}

#[test]
fn test_add_padding_one_byte_padding() {
    let unpadded_output_len = 1;
    let mut output = [0; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 3);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, PAD_BYTE]);
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let unpadded_output_len = 2;
    let mut output = [0; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE]);
}

#[test]
fn test_add_padding_three_bytes_padding() {
    let unpadded_output_len = 3;
    let mut output = [0; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 1);
    assert_eq!(output, [PAD_BYTE, 0]);
}

#[test]
fn test_add_padding_four_bytes_no_padding() {
    let unpadded_output_len = 4;
    let mut output = [0; 2];
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 0);
    assert_eq!(output, [0, 0]);
}


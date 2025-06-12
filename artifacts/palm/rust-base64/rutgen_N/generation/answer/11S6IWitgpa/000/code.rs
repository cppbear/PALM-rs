// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let unpadded_output_len = 4;
    let mut output = [0, 0, 0, 0];
    let padding_count = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_count, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}

#[test]
fn test_add_padding_one_byte_padding() {
    let unpadded_output_len = 5;
    let mut output = [0, 0, 0, 0];
    let padding_count = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_count, 3);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, PAD_BYTE, 0]);
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let unpadded_output_len = 6;
    let mut output = [0, 0, 0, 0];
    let padding_count = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_count, 2);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, 0, 0]);
}

#[test]
fn test_add_padding_three_bytes_padding() {
    let unpadded_output_len = 7;
    let mut output = [0, 0, 0, 0];
    let padding_count = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_count, 1);
    assert_eq!(output, [PAD_BYTE, 0, 0, 0]);
}

#[test]
fn test_add_padding_exact_multiple_of_four() {
    let unpadded_output_len = 8;
    let mut output = [0, 0, 0, 0];
    let padding_count = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_count, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}


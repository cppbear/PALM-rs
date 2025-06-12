// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let unpadded_output_len = 4;
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}

#[test]
fn test_add_padding_one_padding_needed() {
    let unpadded_output_len = 5;
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 3);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, PAD_BYTE, 0]);
}

#[test]
fn test_add_padding_two_padding_needed() {
    let unpadded_output_len = 6;
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 2);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, 0, 0]);
}

#[test]
fn test_add_padding_three_padding_needed() {
    let unpadded_output_len = 7;
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 1);
    assert_eq!(output, [PAD_BYTE, 0, 0, 0]);
}

#[test]
fn test_add_padding_multiple_padding_needed() {
    let unpadded_output_len = 8;
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}


// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let mut output = [0u8; 4];
    let result = add_padding(4, &mut output);
    assert_eq!(result, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}

#[test]
fn test_add_padding_one_byte_padding() {
    let mut output = [0u8; 4];
    let result = add_padding(5, &mut output);
    assert_eq!(result, 3);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, PAD_BYTE, 0]);
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let mut output = [0u8; 4];
    let result = add_padding(6, &mut output);
    assert_eq!(result, 2);
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, 0, 0]);
}

#[test]
fn test_add_padding_three_bytes_padding() {
    let mut output = [0u8; 4];
    let result = add_padding(7, &mut output);
    assert_eq!(result, 1);
    assert_eq!(output, [PAD_BYTE, 0, 0, 0]);
}

#[test]
fn test_add_padding_output_buffer_too_small() {
    // This test ensures we don't actually exceed the bounds of the given slice
    let mut output = [0u8; 2]; // Smaller than required
    let result = add_padding(2, &mut output);
    assert_eq!(result, 2); // We expect it to try writing 2 bytes
    assert_eq!(output, [PAD_BYTE, PAD_BYTE]); // Should pad it with two padding bytes
}

#[test]
#[should_panic]
fn test_add_padding_zero_length_output() {
    let mut output = []; // Empty output slice
    let _ = add_padding(2, &mut output); // This should panic as the buffer is insufficient
}


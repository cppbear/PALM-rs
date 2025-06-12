// Answer 0

#[test]
fn test_vb_with_max_u8() {
    let input: usize = std::u8::MAX as usize;
    let expected_output = String::from("\\xFF");
    let actual_output = vb(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_vb_with_value_below_max_u8() {
    let input: usize = 255;  // One less than max u8
    let expected_output = String::from("\\xFF");
    let actual_output = vb(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_vb_with_value_zero() {
    let input: usize = 0;
    let expected_output = String::from("\\0");
    let actual_output = vb(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_vb_with_non_ascii_value() {
    let input: usize = 128;  // A non-ASCII byte
    let expected_output = String::from("\\x80");
    let actual_output = vb(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_vb_with_value_above_max_u8() {
    let input: usize = std::u8::MAX as usize + 1;
    let expected_output = String::from("EOF");
    let actual_output = vb(input);
    assert_eq!(actual_output, expected_output);
}


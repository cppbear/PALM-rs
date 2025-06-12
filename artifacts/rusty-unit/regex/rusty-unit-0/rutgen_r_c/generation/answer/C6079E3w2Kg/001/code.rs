// Answer 0

#[test]
fn test_vb_above_max_byte() {
    let input = ::std::u8::MAX as usize + 1;
    let expected_output = "EOF".to_owned();
    let result = vb(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_vb_exactly_max_byte() {
    let input = ::std::u8::MAX as usize;
    let expected_output = String::from("\\x7f"); // Escaped representation of 127
    let result = vb(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_vb_zero() {
    let input = 0;
    let expected_output = String::from("\\x00"); // Escaped representation of 0
    let result = vb(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_vb_upper_limit() {
    let input = 255; // This is the last valid byte value
    let expected_output = String::from("\\xff"); // Escaped representation of 255
    let result = vb(input);
    assert_eq!(result, expected_output);
}


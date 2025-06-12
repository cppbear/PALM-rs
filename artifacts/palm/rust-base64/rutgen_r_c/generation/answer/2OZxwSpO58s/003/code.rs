// Answer 0

#[test]
fn test_invalid_length_display() {
    let error = DecodeError::InvalidLength(5);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 5");
}

#[test]
fn test_invalid_length_display_zero() {
    let error = DecodeError::InvalidLength(0);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 0");
}


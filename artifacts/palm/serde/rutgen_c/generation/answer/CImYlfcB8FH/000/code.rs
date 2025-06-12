// Answer 0

#[test]
fn test_visit_byte_buf_success() {
    let mut string_value = String::new();
    let visitor = StringInPlaceVisitor(&mut string_value);
    let input = b"valid utf8".to_vec();

    let result: Result<(), _> = visitor.visit_byte_buf(input);

    assert!(result.is_ok());
    assert_eq!(string_value, "valid utf8");
}

#[test]
fn test_visit_byte_buf_invalid_utf8() {
    let mut string_value = String::new();
    let visitor = StringInPlaceVisitor(&mut string_value);
    let input = vec![0, 159, 146, 150]; // Invalid UTF-8 sequence

    let result: Result<(), _> = visitor.visit_byte_buf(input);

    assert!(result.is_err());
    assert_eq!(string_value, ""); // Ensure the string is still empty
}


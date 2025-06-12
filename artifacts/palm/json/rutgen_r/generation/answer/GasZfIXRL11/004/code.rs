// Answer 0

#[test]
fn test_peek_invalid_type_null() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'n'); // set up the state for null
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidType(Unexpected::Unit));
}

#[test]
fn test_peek_invalid_type_array() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'['); // set up the state for array
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidType(Unexpected::Seq));
}

#[test]
fn test_peek_invalid_type_string() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'"'); // set up the state for string
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidType(Unexpected::Str("")));
}

#[test]
fn test_peek_invalid_type_false() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'f'); // set up the state for false
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidType(Unexpected::Bool(false)));
}

#[test]
fn test_peek_invalid_type_object() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'{'); // set up the state for object
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidType(Unexpected::Map));
}

#[test]
fn test_peek_invalid_type_true() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b't'); // set up the state for true
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidType(Unexpected::Bool(true)));
}

#[test]
fn test_peek_invalid_type_negative_number() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'-'); // set up the state for negative number
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err()); // Expecting an error due to invalid type
}

#[test]
fn test_peek_invalid_type_positive_number() {
    struct ExpectedImpl;
    impl Expected for ExpectedImpl {}

    let mut de = MyDe::new();
    de.set_peek(b'5'); // set up the state for positive number
    let result = de.peek_invalid_type(&ExpectedImpl);
    assert!(result.is_err()); // Expecting an error due to invalid type
}


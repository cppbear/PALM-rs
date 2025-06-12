// Answer 0

#[test]
fn test_invalid_type_f64() {
    struct DummyExpected;

    impl Expected for DummyExpected {}

    let number = ParserNumber::F64(42.0);
    let error = number.invalid_type(&DummyExpected);
    
    match error {
        de::Error::InvalidType(Unexpected::Float(x), _) => assert_eq!(x, 42.0),
        _ => panic!("Expected InvalidType with Float but got a different error."),
    }
}

#[test]
fn test_invalid_type_u64() {
    struct DummyExpected;

    impl Expected for DummyExpected {}

    let number = ParserNumber::U64(42);
    let error = number.invalid_type(&DummyExpected);
    
    match error {
        de::Error::InvalidType(Unexpected::Unsigned(x), _) => assert_eq!(x, 42),
        _ => panic!("Expected InvalidType with Unsigned but got a different error."),
    }
}

#[test]
fn test_invalid_type_i64() {
    struct DummyExpected;

    impl Expected for DummyExpected {}

    let number = ParserNumber::I64(-42);
    let error = number.invalid_type(&DummyExpected);
    
    match error {
        de::Error::InvalidType(Unexpected::Signed(x), _) => assert_eq!(x, -42),
        _ => panic!("Expected InvalidType with Signed but got a different error."),
    }
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_invalid_type_string() {
    struct DummyExpected;

    impl Expected for DummyExpected {}

    let number = ParserNumber::String("not_a_number".to_string());
    let error = number.invalid_type(&DummyExpected);
    
    match error {
        de::Error::InvalidType(Unexpected::Other(ref s), _) => assert_eq!(s, "number"),
        _ => panic!("Expected InvalidType with Other but got a different error."),
    }
}


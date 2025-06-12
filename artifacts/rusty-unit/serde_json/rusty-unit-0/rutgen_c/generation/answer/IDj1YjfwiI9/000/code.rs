// Answer 0

#[test]
fn test_invalid_type_f64() {
    let num = ParserNumber::F64(3.14);
    let exp = &Unexpected::Float(3.14); // Mocking the Expected
    let error = num.invalid_type(exp);
    match error.err.downcast_ref::<Unexpected>() {
        Some(Unexpected::Float(x)) => assert_eq!(*x, 3.14),
        _ => panic!("Expected error type does not match"),
    }
}

#[test]
fn test_invalid_type_u64() {
    let num = ParserNumber::U64(42);
    let exp = &Unexpected::Unsigned(42); // Mocking the Expected
    let error = num.invalid_type(exp);
    match error.err.downcast_ref::<Unexpected>() {
        Some(Unexpected::Unsigned(x)) => assert_eq!(*x, 42),
        _ => panic!("Expected error type does not match"),
    }
}

#[test]
fn test_invalid_type_i64() {
    let num = ParserNumber::I64(-42);
    let exp = &Unexpected::Signed(-42); // Mocking the Expected
    let error = num.invalid_type(exp);
    match error.err.downcast_ref::<Unexpected>() {
        Some(Unexpected::Signed(x)) => assert_eq!(*x, -42),
        _ => panic!("Expected error type does not match"),
    }
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_invalid_type_string() {
    let num = ParserNumber::String("42".to_string());
    let exp = &Unexpected::Other("number"); // Mocking the Expected
    let error = num.invalid_type(exp);
    match error.err.downcast_ref::<Unexpected>() {
        Some(Unexpected::Other(msg)) => assert_eq!(msg, "number"),
        _ => panic!("Expected error type does not match"),
    }
}


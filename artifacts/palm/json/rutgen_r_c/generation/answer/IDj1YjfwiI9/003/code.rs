// Answer 0

#[test]
fn test_invalid_type_f64() {
    let num = ParserNumber::F64(3.14);
    let exp = &Unexpected::Other("float");
    let error = num.invalid_type(exp);
    match error {
        Error::InvalidType(Unexpected::Float(x), _) => assert_eq!(x, 3.14),
        _ => panic!("Expected invalid type for float, got {:?}", error),
    }
}

#[test]
fn test_invalid_type_u64() {
    let num = ParserNumber::U64(42);
    let exp = &Unexpected::Other("unsigned");
    let error = num.invalid_type(exp);
    match error {
        Error::InvalidType(Unexpected::Unsigned(x), _) => assert_eq!(x, 42),
        _ => panic!("Expected invalid type for unsigned int, got {:?}", error),
    }
}

#[test]
fn test_invalid_type_i64() {
    let num = ParserNumber::I64(-7);
    let exp = &Unexpected::Other("signed");
    let error = num.invalid_type(exp);
    match error {
        Error::InvalidType(Unexpected::Signed(x), _) => assert_eq!(x, -7),
        _ => panic!("Expected invalid type for signed int, got {:?}", error),
    }
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_invalid_type_string() {
    let num = ParserNumber::String("1000".to_string());
    let exp = &Unexpected::Other("number");
    let error = num.invalid_type(exp);
    match error {
        Error::InvalidType(Unexpected::Other("number"), _) => {},
        _ => panic!("Expected invalid type for string, got {:?}", error),
    }
}


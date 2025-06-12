// Answer 0

#[test]
fn test_serialize_i128_u64_conversion() {
    let value: i128 = 42; // This should pass u64::try_from(value)
    let result = serialize_i128(value);
    assert_eq!(result, Ok(Value::Number(42u64.into())));
}

#[test]
fn test_serialize_i128_i64_conversion() {
    let value: i128 = -42; // This should pass i64::try_from(value)
    let result = serialize_i128(value);
    assert_eq!(result, Ok(Value::Number((-42i64).into())));
}

#[test]
#[should_panic]
fn test_serialize_i128_out_of_range() {
    let value: i128 = 2_i128.pow(128); // This should exceed both u64 and i64
    let result = serialize_i128(value);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, Error::syntax(ErrorCode::NumberOutOfRange, 0, 0));
    }
}

#[test]
#[should_panic]
fn test_serialize_i128_negative_out_of_range() {
    let value: i128 = -2_i128.pow(128); // This should exceed the negative range of i64
    let result = serialize_i128(value);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, Error::syntax(ErrorCode::NumberOutOfRange, 0, 0));
    }
}


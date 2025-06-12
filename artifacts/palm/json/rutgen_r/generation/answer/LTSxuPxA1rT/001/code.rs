// Answer 0

#[test]
fn test_serialize_i128_u64_conversion() {
    let value: i128 = 18446744073709551615; // Maximum u64 value
    let result = serialize_i128(value);
    assert!(result.is_ok());
    if let Ok(val) = result {
        assert_eq!(val, Value::Number(18446744073709551615u64.into()));
    }
}

#[test]
fn test_serialize_i128_i64_conversion() {
    let value: i128 = 9223372036854775807; // Maximum i64 value
    let result = serialize_i128(value);
    assert!(result.is_ok());
    if let Ok(val) = result {
        assert_eq!(val, Value::Number(9223372036854775807i64.into()));
    }
}

#[test]
fn test_serialize_i128_out_of_range() {
    let value: i128 = 9223372036854775808; // One more than max i64 value
    let result = serialize_i128(value);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err, Error::syntax(ErrorCode::NumberOutOfRange, 0, 0));
    }
}


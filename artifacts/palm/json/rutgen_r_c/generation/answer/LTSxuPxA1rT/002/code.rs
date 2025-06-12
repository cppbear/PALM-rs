// Answer 0

#[test]
fn test_serialize_i128_within_u64_range() {
    let serializer = Serializer;
    let value: i128 = 18446744073709551615; // maximum value of u64
    let result = serializer.serialize_i128(value);
    assert_eq!(
        result,
        Ok(Value::Number(Number { n: value.into() }))
    );
}

#[test]
fn test_serialize_i128_exceeding_i64_range() {
    let serializer = Serializer;
    let value: i128 = 9223372036854775808; // one more than the max value of i64
    let result = serializer.serialize_i128(value);
    assert_eq!(
        result,
        Ok(Value::Number(Number { n: value.into() }))
    );
}

#[test]
fn test_serialize_i128_exceeding_u64_range() {
    let serializer = Serializer;
    let value: i128 = 18446744073709551616; // one more than the max value of u64
    let result = serializer.serialize_i128(value);
    assert_eq!(
        result,
        Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
    );
}

#[test]
fn test_serialize_i128_negative_value() {
    let serializer = Serializer;
    let value: i128 = -1; // a negative value
    let result = serializer.serialize_i128(value);
    assert_eq!(
        result,
        Ok(Value::Number(Number { n: value.into() }))
    );
}

#[test]
fn test_serialize_i128_zero() {
    let serializer = Serializer;
    let value: i128 = 0; // boundary value of zero
    let result = serializer.serialize_i128(value);
    assert_eq!(
        result,
        Ok(Value::Number(Number { n: value.into() }))
    );
}


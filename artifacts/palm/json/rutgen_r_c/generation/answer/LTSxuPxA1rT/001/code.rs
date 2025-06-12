// Answer 0

#[test]
fn test_serialize_i128_within_u64_bounds() {
    let serializer = Serializer;

    // Test input that satisfies the u64 constraint
    let value: i128 = 18446744073709551615; // Max u64 value
    assert_eq!(serializer.serialize_i128(value), Ok(Value::Number(Number { n: value.into() })));
}

#[test]
fn test_serialize_i128_within_i64_bounds() {
    let serializer = Serializer;

    // Test input that satisfies the i64 constraint
    let value: i128 = 9223372036854775807; // Max i64 value
    assert_eq!(serializer.serialize_i128(value), Ok(Value::Number(Number { n: value.into() })));
}

#[test]
fn test_serialize_i128_out_of_bounds() {
    let serializer = Serializer;

    // Test input that exceeds both u64 and i64 bounds
    let value: i128 = 9223372036854775808; // Out of i64 bounds
    assert_eq!(serializer.serialize_i128(value), Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0)));
}

#[test]
fn test_serialize_i128_negative_out_of_bounds() {
    let serializer = Serializer;

    // Test input that exceeds i64 negative bounds
    let value: i128 = -9223372036854775809; // Out of i64 negative bounds
    assert_eq!(serializer.serialize_i128(value), Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0)));
}


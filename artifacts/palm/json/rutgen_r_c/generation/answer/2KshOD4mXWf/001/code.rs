// Answer 0

#[test]
fn test_serialize_i64_positive() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(64);
    assert_eq!(result, Ok(Value::Number(Number { n: 64.into() })));
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(-64);
    assert_eq!(result, Ok(Value::Number(Number { n: (-64).into() })));
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(0);
    assert_eq!(result, Ok(Value::Number(Number { n: 0.into() })));
}


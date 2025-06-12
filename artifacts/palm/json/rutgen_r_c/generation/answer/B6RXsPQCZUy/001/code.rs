// Answer 0

#[test]
fn test_serialize_u128_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u128(0u128);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_u128_small_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u128(42u128);
    assert_eq!(result, Ok("42".to_owned()));
}

#[test]
fn test_serialize_u128_large_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u128(123456789012345678901234567890u128);
    assert_eq!(result, Ok("123456789012345678901234567890".to_owned()));
}

#[test]
fn test_serialize_u128_max_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u128(u128::MAX);
    assert_eq!(result, Ok("340282366920938463463374607431768211455".to_owned()));
}


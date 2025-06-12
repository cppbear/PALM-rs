// Answer 0

#[test]
fn test_serialize_i128_positive() {
    let serializer = MapKeySerializer;
    let value: i128 = 12345678901234567890;
    let result = serializer.serialize_i128(value);
    assert_eq!(result, Ok("12345678901234567890".to_owned()));
}

#[test]
fn test_serialize_i128_negative() {
    let serializer = MapKeySerializer;
    let value: i128 = -12345678901234567890;
    let result = serializer.serialize_i128(value);
    assert_eq!(result, Ok("-12345678901234567890".to_owned()));
}

#[test]
fn test_serialize_i128_zero() {
    let serializer = MapKeySerializer;
    let value: i128 = 0;
    let result = serializer.serialize_i128(value);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_i128_min_value() {
    let serializer = MapKeySerializer;
    let value: i128 = i128::MIN;
    let result = serializer.serialize_i128(value);
    assert_eq!(result, Ok("-170141183460469231731687303715884105728".to_owned()));
}

#[test]
fn test_serialize_i128_max_value() {
    let serializer = MapKeySerializer;
    let value: i128 = i128::MAX;
    let result = serializer.serialize_i128(value);
    assert_eq!(result, Ok("170141183460469231731687303715884105727".to_owned()));
}


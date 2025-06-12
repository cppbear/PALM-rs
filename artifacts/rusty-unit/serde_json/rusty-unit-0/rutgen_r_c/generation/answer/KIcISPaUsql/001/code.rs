// Answer 0

#[test]
fn test_serialize_u64_zero() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u64(0);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_u64_small_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u64(5);
    assert_eq!(result, Ok("5".to_owned()));
}

#[test]
fn test_serialize_u64_large_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u64(123456789);
    assert_eq!(result, Ok("123456789".to_owned()));
}

#[test]
fn test_serialize_u64_boundary_value() {
    let serializer = MapKeySerializer;
    let result = serializer.serialize_u64(u64::MAX);
    assert_eq!(result, Ok("18446744073709551615".to_owned()));
}


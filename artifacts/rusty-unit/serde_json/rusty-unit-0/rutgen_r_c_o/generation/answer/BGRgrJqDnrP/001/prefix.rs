// Answer 0

#[test]
fn test_serialize_i64_negative_value() {
    let serializer = MapKeySerializer;
    let value = -1i64;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_zero_value() {
    let serializer = MapKeySerializer;
    let value = 0i64;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_positive_value() {
    let serializer = MapKeySerializer;
    let value = 1i64;
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_large_positive_value() {
    let serializer = MapKeySerializer;
    let value = 9223372036854775807i64; // max i64 value
    let _result = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_large_negative_value() {
    let serializer = MapKeySerializer;
    let value = -9223372036854775807i64; // just below max i64 value
    let _result = serializer.serialize_i64(value);
}


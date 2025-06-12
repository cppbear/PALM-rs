// Answer 0

#[test]
fn test_serialize_i64_min() {
    let serializer = Serializer;
    let value = i64::MIN;
    let _ = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_negative() {
    let serializer = Serializer;
    let value = -12345678901234;
    let _ = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = Serializer;
    let value = 0;
    let _ = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_positive() {
    let serializer = Serializer;
    let value = 12345678901234;
    let _ = serializer.serialize_i64(value);
}

#[test]
fn test_serialize_i64_max() {
    let serializer = Serializer;
    let value = i64::MAX;
    let _ = serializer.serialize_i64(value);
}


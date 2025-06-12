// Answer 0

#[test]
fn test_serialize_i32_min() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(-2147483648);
}

#[test]
fn test_serialize_i32_negative() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(-1);
}

#[test]
fn test_serialize_i32_zero() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_positive() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(1);
}

#[test]
fn test_serialize_i32_max() {
    let serializer = Serializer;
    let _result = serializer.serialize_i32(2147483647);
}


// Answer 0

#[test]
fn test_serialize_i32_min_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i32(i32::MIN);
}

#[test]
fn test_serialize_i32_zero() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i32(0);
}

#[test]
fn test_serialize_i32_positive_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i32(123456);
}

#[test]
fn test_serialize_i32_max_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i32(i32::MAX);
}

#[test]
fn test_serialize_i32_negative_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::new();
    let mut serializer = Serializer { writer, formatter };
    serializer.serialize_i32(-123456);
}


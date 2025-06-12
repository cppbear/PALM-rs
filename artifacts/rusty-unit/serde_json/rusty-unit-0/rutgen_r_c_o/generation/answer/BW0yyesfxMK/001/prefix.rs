// Answer 0

#[test]
fn test_serialize_i128_min() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = i128::MIN;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_max() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = i128::MAX;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_negative_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = -123456789012345678901234567890i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_positive_value() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = 123456789012345678901234567890i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_zero() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = 0i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_small_negative() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = -1i128;
    serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_small_positive() {
    let writer = Vec::new();
    let formatter = CompactFormatter::default();
    let mut serializer = Serializer { writer, formatter };
    let value = 1i128;
    serializer.serialize_i128(value);
}


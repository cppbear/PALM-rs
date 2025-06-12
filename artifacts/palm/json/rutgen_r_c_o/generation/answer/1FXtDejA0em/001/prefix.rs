// Answer 0

#[test]
fn test_serialize_i8_min_value() {
    let serializer = Serializer;
    let _ = serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_negative_value() {
    let serializer = Serializer;
    let _ = serializer.serialize_i8(-1);
}

#[test]
fn test_serialize_i8_zero_value() {
    let serializer = Serializer;
    let _ = serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_positive_value() {
    let serializer = Serializer;
    let _ = serializer.serialize_i8(1);
}

#[test]
fn test_serialize_i8_max_value() {
    let serializer = Serializer;
    let _ = serializer.serialize_i8(127);
}


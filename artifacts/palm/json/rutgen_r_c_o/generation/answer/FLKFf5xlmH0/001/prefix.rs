// Answer 0

#[test]
fn test_serialize_u8_0() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(0);
}

#[test]
fn test_serialize_u8_1() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(1);
}

#[test]
fn test_serialize_u8_255() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(255);
}

#[test]
fn test_serialize_u8_mid_value() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(128);
}

#[test]
fn test_serialize_u8_large_value() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(200);
}


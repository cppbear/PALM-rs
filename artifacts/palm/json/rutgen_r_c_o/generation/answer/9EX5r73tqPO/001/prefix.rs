// Answer 0

#[test]
fn test_serialize_u16_min() {
    let serializer = Serializer;
    let _ = serializer.serialize_u16(0);
}

#[test]
fn test_serialize_u16_max() {
    let serializer = Serializer;
    let _ = serializer.serialize_u16(65535);
}

#[test]
fn test_serialize_u16_mid() {
    let serializer = Serializer;
    let _ = serializer.serialize_u16(32768);
}

#[test]
fn test_serialize_u16_near_min() {
    let serializer = Serializer;
    let _ = serializer.serialize_u16(1);
}

#[test]
fn test_serialize_u16_near_max() {
    let serializer = Serializer;
    let _ = serializer.serialize_u16(65534);
}

#[test]
fn test_serialize_u16_sequence() {
    let serializer = Serializer;
    for value in [2, 3, 4, 5, 10, 15, 100, 250, 500, 1000, 20000, 65000].iter() {
        let _ = serializer.serialize_u16(*value);
    }
}


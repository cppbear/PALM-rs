// Answer 0

#[test]
fn test_serialize_bytes_non_empty_slice() {
    let serializer = MapKeySerializer;
    let value: &[u8] = &[1, 2, 3];
    let _ = serializer.serialize_bytes(value);
}

#[test]
fn test_serialize_bytes_empty_slice() {
    let serializer = MapKeySerializer;
    let value: &[u8] = &[];
    let _ = serializer.serialize_bytes(value);
}

#[test]
fn test_serialize_bytes_single_element_slice() {
    let serializer = MapKeySerializer;
    let value: &[u8] = &[10];
    let _ = serializer.serialize_bytes(value);
}


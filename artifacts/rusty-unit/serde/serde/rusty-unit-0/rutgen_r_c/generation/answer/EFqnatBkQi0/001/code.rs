// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_bytes(&[]);
    assert_eq!(result, Ok(Content::Bytes(Vec::new())));
}

#[test]
fn test_serialize_bytes_single_byte() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_bytes(&[42]);
    assert_eq!(result, Ok(Content::Bytes(vec![42])));
}

#[test]
fn test_serialize_bytes_multiple_bytes() {
    let serializer = ContentSerializer::<()>::default();
    let data = &[1, 2, 3, 4, 5];
    let result = serializer.serialize_bytes(data);
    assert_eq!(result, Ok(Content::Bytes(data.to_vec())));
}

#[test]
fn test_serialize_bytes_large_data() {
    let serializer = ContentSerializer::<()>::default();
    let data = &[0u8; 1024]; // 1 KB of data
    let result = serializer.serialize_bytes(data);
    assert_eq!(result, Ok(Content::Bytes(data.to_vec())));
}


// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let serializer = ContentSerializer::<()>::default();
    let input: &[u8] = &[];
    let _ = serializer.serialize_bytes(input);
}

#[test]
fn test_serialize_bytes_single_byte() {
    let serializer = ContentSerializer::<()>::default();
    let input: &[u8] = &[0];
    let _ = serializer.serialize_bytes(input);
}

#[test]
fn test_serialize_bytes_max_single_byte() {
    let serializer = ContentSerializer::<()>::default();
    let input: &[u8] = &[255];
    let _ = serializer.serialize_bytes(input);
}

#[test]
fn test_serialize_bytes_multiple_bytes() {
    let serializer = ContentSerializer::<()>::default();
    let input: &[u8] = &[1, 2, 3, 4, 5];
    let _ = serializer.serialize_bytes(input);
}

#[test]
fn test_serialize_bytes_large_input() {
    let serializer = ContentSerializer::<()>::default();
    let input: Vec<u8> = (1..=1048576).map(|x| (x % 256) as u8).collect();
    let _ = serializer.serialize_bytes(&input);
}


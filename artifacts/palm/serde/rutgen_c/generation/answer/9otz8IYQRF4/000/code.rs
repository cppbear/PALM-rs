// Answer 0

#[test]
fn test_bytes_deserializer_new() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let deserializer = BytesDeserializer::new(data);
    assert_eq!(deserializer.value, data);
}

#[test]
fn test_bytes_deserializer_with_empty_array() {
    let data: &[u8] = &[];
    let deserializer = BytesDeserializer::new(data);
    assert_eq!(deserializer.value, data);
}

#[test]
fn test_bytes_deserializer_with_large_array() {
    let data: &[u8] = &[0; 1_000]; // large array of zeros
    let deserializer = BytesDeserializer::new(data);
    assert_eq!(deserializer.value, data);
}


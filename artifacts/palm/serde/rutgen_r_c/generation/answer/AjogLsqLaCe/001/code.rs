// Answer 0

#[test]
fn test_borrowed_bytes_deserializer_new() {
    let bytes: &[u8] = &[1, 2, 3];
    let deserializer = BorrowedBytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}

#[test]
fn test_borrowed_bytes_deserializer_empty() {
    let bytes: &[u8] = &[];
    let deserializer = BorrowedBytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}

#[test]
fn test_borrowed_bytes_deserializer_single_byte() {
    let bytes: &[u8] = &[42];
    let deserializer = BorrowedBytesDeserializer::new(bytes);
    assert_eq!(deserializer.value, bytes);
}


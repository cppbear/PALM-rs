// Answer 0

#[test]
fn test_clear_empty_bytes() {
    let mut buf = Bytes::new();
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_non_empty_bytes() {
    let mut buf = Bytes::from_static(&[1, 2, 3, 4]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_bytes_after_mutation() {
    let mut buf = Bytes::from_static(&[5, 6, 7, 8, 9]);
    buf.truncate(3); // Mutate before clearing
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_large_bytes() {
    let large_data = vec![0u8; 1000]; // Large byte array
    let mut buf = Bytes::from(large_data);
    buf.clear();
    assert!(buf.is_empty());
}


// Answer 0

#[test]
fn test_deref_empty_bytes() {
    let bytes = Bytes::new();
    let slice: &[u8] = &*bytes;
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_deref_non_empty_bytes() {
    let static_bytes: &'static [u8] = &[1, 2, 3, 4, 5];
    let bytes = Bytes::from_static(static_bytes);
    let slice: &[u8] = &*bytes;
    assert_eq!(slice.len(), 5);
    assert_eq!(slice, &[1, 2, 3, 4, 5]);
}


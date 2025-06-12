// Answer 0

#[test]
fn test_bytes_mut_is_empty_with_new() {
    let bytes_mut = BytesMut::new();
    assert!(bytes_mut.is_empty());
}

#[test]
fn test_bytes_mut_is_empty_with_capacity() {
    let bytes_mut = BytesMut::with_capacity(64);
    assert!(bytes_mut.is_empty());
}

#[test]
fn test_bytes_mut_is_empty_after_resize() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.resize(0, 0);
    assert!(bytes_mut.is_empty());
}

#[test]
fn test_bytes_mut_is_empty_after_truncate() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.resize(10, 0);
    bytes_mut.truncate(0);
    assert!(bytes_mut.is_empty());
}


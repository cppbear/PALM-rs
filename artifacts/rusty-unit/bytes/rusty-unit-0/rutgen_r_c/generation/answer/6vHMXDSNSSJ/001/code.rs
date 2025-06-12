// Answer 0

#[test]
fn test_len_with_empty_bytes_mut() {
    let bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.len(), 0);
}

#[test]
fn test_len_with_non_empty_bytes_mut() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(bytes_mut.len(), 5);
}

#[test]
fn test_len_after_truncate() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes_mut.truncate(3);
    assert_eq!(bytes_mut.len(), 3);
}

#[test]
fn test_len_after_resize() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.resize(5, 0);
    assert_eq!(bytes_mut.len(), 5);
}

#[test]
fn test_len_after_clear() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3]);
    bytes_mut.clear();
    assert_eq!(bytes_mut.len(), 0);
}


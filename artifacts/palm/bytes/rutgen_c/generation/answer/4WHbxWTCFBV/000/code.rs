// Answer 0

#[test]
fn test_put_bytes_single_write() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.put_bytes(5, 1);
    assert_eq!(bytes_mut.len(), 1);
    assert_eq!(bytes_mut.as_slice()[0], 5);
}

#[test]
fn test_put_bytes_multiple_writes() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.put_bytes(7, 3);
    assert_eq!(bytes_mut.len(), 3);
    assert_eq!(bytes_mut.as_slice()[0], 7);
    assert_eq!(bytes_mut.as_slice()[1], 7);
    assert_eq!(bytes_mut.as_slice()[2], 7);
}

#[test]
fn test_put_bytes_exceeding_initial_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.put_bytes(10, 6);
    assert_eq!(bytes_mut.len(), 6);
    for &byte in bytes_mut.as_slice().iter().take(6) {
        assert_eq!(byte, 10);
    }
}

#[test]
fn test_put_bytes_zero_count() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.put_bytes(3, 0);
    assert_eq!(bytes_mut.len(), 0);
}


// Answer 0

#[test]
fn test_zeroed_with_zero_length() {
    let bytes = BytesMut::zeroed(0);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.capacity() >= 0);
}

#[test]
fn test_zeroed_with_small_length() {
    let bytes = BytesMut::zeroed(1);
    assert_eq!(bytes.len(), 1);
    assert!(bytes.capacity() >= 1);
    let slice = unsafe { bytes.as_slice() };
    assert_eq!(slice[0], 0);
}

#[test]
fn test_zeroed_with_large_length() {
    let bytes = BytesMut::zeroed(100);
    assert_eq!(bytes.len(), 100);
    assert!(bytes.capacity() >= 100);
    let slice = unsafe { bytes.as_slice() };
    for &byte in slice {
        assert_eq!(byte, 0);
    }
}

#[test]
fn test_zeroed_with_edge_capacity() {
    let length = usize::max_value();
    let bytes = BytesMut::zeroed(length);
    assert_eq!(bytes.len(), length);
    assert!(bytes.capacity() >= length);
}


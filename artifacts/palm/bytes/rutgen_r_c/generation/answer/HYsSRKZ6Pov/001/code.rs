// Answer 0

#[test]
fn test_capacity_with_non_zero_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(128);
    assert_eq!(bytes_mut.capacity(), 128);
}

#[test]
fn test_capacity_with_zero_capacity() {
    let bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.capacity(), 0);
}

#[test]
fn test_capacity_after_reserve() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.reserve(32);
    assert_eq!(bytes_mut.capacity(), 64); // Ensure capacity remains the same after reserve
}

#[test]
fn test_capacity_with_large_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    assert_eq!(bytes_mut.capacity(), usize::MAX);
}

#[test]
#[should_panic]
fn test_capacity_with_negative_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    bytes_mut.resize(usize::MAX - 1, 0);
    bytes_mut.truncate(usize::MAX); // This should panic
}


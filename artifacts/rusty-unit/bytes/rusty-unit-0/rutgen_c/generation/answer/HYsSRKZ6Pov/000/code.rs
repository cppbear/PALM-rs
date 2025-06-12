// Answer 0

#[test]
fn test_capacity_with_initial_capacity() {
    let b = BytesMut::with_capacity(64);
    assert_eq!(b.capacity(), 64);
}

#[test]
fn test_capacity_after_creation() {
    let b = BytesMut::new();
    assert_eq!(b.capacity(), 0);
}

#[test]
fn test_capacity_increased_after_reserve() {
    let mut b = BytesMut::new();
    b.reserve(32);
    assert!(b.capacity() >= 32);
}

#[test]
fn test_capacity_zeroed() {
    let b = BytesMut::zeroed(10);
    assert_eq!(b.capacity(), 10);
}


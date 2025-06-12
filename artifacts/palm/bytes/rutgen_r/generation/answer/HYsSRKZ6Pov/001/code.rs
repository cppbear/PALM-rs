// Answer 0

#[test]
fn test_capacity_with_standard_value() {
    let b = BytesMut::with_capacity(64);
    assert_eq!(b.capacity(), 64);
}

#[test]
fn test_capacity_with_zero() {
    let b = BytesMut::with_capacity(0);
    assert_eq!(b.capacity(), 0);
}

#[test]
fn test_capacity_with_large_value() {
    let b = BytesMut::with_capacity(1024);
    assert_eq!(b.capacity(), 1024);
}

#[test]
fn test_capacity_with_small_value() {
    let b = BytesMut::with_capacity(1);
    assert_eq!(b.capacity(), 1);
}

#[test]
#[should_panic]
fn test_capacity_creation_failure() {
    let b: BytesMut = BytesMut::with_capacity(usize::MAX);
    assert_eq!(b.capacity(), usize::MAX);
}


// Answer 0

#[test]
fn test_with_capacity_non_zero() {
    let bytes = BytesMut::with_capacity(64);
    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), 64);
}

#[test]
fn test_with_capacity_zero() {
    let bytes = BytesMut::with_capacity(0);
    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), 0);
}

#[test]
fn test_with_capacity_large_value() {
    let large_capacity = usize::MAX; // Test maximum capacity to check behavior
    let bytes = BytesMut::with_capacity(large_capacity);
    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.capacity(), large_capacity);
}

#[test]
fn test_with_capacity_small_value() {
    let small_capacity = 1; // Test small capacity
    let bytes = BytesMut::with_capacity(small_capacity);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.capacity() >= small_capacity);
}


// Answer 0

#[test]
fn test_capacity_with_default_initialization() {
    let b = BytesMut { cap: 0 }; // assuming default initialization
    assert_eq!(b.capacity(), 0);
}

#[test]
fn test_capacity_with_custom_capacity() {
    let b = BytesMut { cap: 64 };
    assert_eq!(b.capacity(), 64);
}

#[test]
fn test_capacity_after_increasing_capacity() {
    let mut b = BytesMut { cap: 32 };
    b.cap = 128; // simulate increasing capacity
    assert_eq!(b.capacity(), 128);
}

#[test]
fn test_capacity_zero_edge_case() {
    let b = BytesMut { cap: 0 };
    assert_eq!(b.capacity(), 0);
}

#[test]
fn test_capacity_large_value() {
    let b = BytesMut { cap: usize::MAX };
    assert_eq!(b.capacity(), usize::MAX);
}


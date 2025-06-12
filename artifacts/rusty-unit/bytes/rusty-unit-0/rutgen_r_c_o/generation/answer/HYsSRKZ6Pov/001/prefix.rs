// Answer 0

#[test]
fn test_capacity_zero() {
    let b = BytesMut::new();
    let _ = b.capacity();
}

#[test]
fn test_capacity_non_zero() {
    let capacity: usize = 128;
    let b = BytesMut::with_capacity(capacity);
    let _ = b.capacity();
}

#[test]
fn test_capacity_large_value() {
    let capacity: usize = usize::MAX;
    let b = BytesMut::with_capacity(capacity);
    let _ = b.capacity();
}

#[test]
fn test_capacity_initialization() {
    let default_capacity: usize = 16;
    let b = BytesMut::with_capacity(default_capacity);
    let _ = b.capacity();
    
    let b_empty = BytesMut::new();
    let _ = b_empty.capacity();
}


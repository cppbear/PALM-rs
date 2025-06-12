// Answer 0

#[test]
fn test_reserve_with_exact_capacity() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(&[0; 64][..]);
    
    let initial_capacity = buf.capacity();
    let additional = initial_capacity - buf.len(); // additional == rem

    buf.reserve(additional);

    assert_eq!(buf.capacity(), initial_capacity);
}

#[test]
fn test_reserve_with_zero_additional() {
    let mut buf = BytesMut::new();
    
    buf.reserve(0);

    assert_eq!(buf.capacity(), 0);
}

#[test]
fn test_reserve_with_additional_equal_to_remaining_capacity() {
    let mut buf = BytesMut::with_capacity(128);
    buf.extend_from_slice(&[0; 64][..]);
    
    let rem = buf.capacity() - buf.len(); // rem
    buf.reserve(rem);

    assert_eq!(buf.capacity(), 128); // should not resize
}

#[test]
#[should_panic]
fn test_reserve_panic_on_capacity_overflow() {
    let mut buf = BytesMut::with_capacity(usize::MAX);
    
    // This should trigger a panic due to capacity overflow.
    buf.reserve(1);
}

#[test]
fn test_reserve_with_non_zero_additional_when_empty() {
    let mut buf = BytesMut::new();

    buf.reserve(64); // should allocate new space, since it's empty
    assert!(buf.capacity() >= 64);
}


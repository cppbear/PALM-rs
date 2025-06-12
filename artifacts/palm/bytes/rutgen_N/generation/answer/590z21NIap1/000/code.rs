// Answer 0

#[test]
fn test_advance_unchecked_zero_count() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.advance_unchecked(0);
    }
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.cap, 10);
}

#[test]
fn test_advance_unchecked_within_capacity() {
    let mut buffer = BytesMut::with_capacity(10);
    buffer.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    unsafe {
        buffer.advance_unchecked(3);
    }
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.cap, 10);
}

#[test]
fn test_advance_unchecked_at_capacity() {
    let mut buffer = BytesMut::with_capacity(10);
    buffer.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    unsafe {
        buffer.advance_unchecked(5);
    }
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.cap, 10);
}

#[should_panic]
fn test_advance_unchecked_exceeds_capacity() {
    let mut buffer = BytesMut::with_capacity(10);
    buffer.extend_from_slice(&[1, 2, 3, 4, 5]);

    unsafe {
        buffer.advance_unchecked(11); // This should panic
    }
}


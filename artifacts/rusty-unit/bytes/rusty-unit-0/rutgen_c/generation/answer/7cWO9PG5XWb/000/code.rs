// Answer 0

#[test]
fn test_reserve_when_capacity_is_sufficient() {
    let mut buf = BytesMut::with_capacity(128);
    let initial_capacity = buf.capacity();
    buf.reserve(64);
    assert!(buf.capacity() >= initial_capacity);
}

#[test]
fn test_reserve_when_buffer_is_empty() {
    let mut buf = BytesMut::new();
    buf.reserve(128);
    assert_eq!(buf.capacity(), 128);
}

#[test]
fn test_reserve_when_reclaiming_space() {
    let mut buf = BytesMut::with_capacity(128);
    buf.extend_from_slice(&[0; 64]);
    let ptr = buf.as_ptr();
    let other = buf.split();
    
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 64);
    
    drop(other);
    buf.reserve(128);
    
    assert_eq!(buf.capacity(), 128);
    assert_eq!(buf.as_ptr(), ptr);
}

#[test]
#[should_panic]
fn test_reserve_panics_on_capacity_overflow() {
    let mut buf = BytesMut::new();
    buf.reserve(usize::MAX); // This should panic due to overflow
}

#[test]
fn test_reserve_with_zero() {
    let mut buf = BytesMut::with_capacity(64);
    let initial_capacity = buf.capacity();
    buf.reserve(0);
    assert_eq!(buf.capacity(), initial_capacity);
}


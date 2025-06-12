// Answer 0

#[test]
fn test_advance_unchecked_noop() {
    let mut buffer = unsafe { BytesMut::with_capacity(10) };
    unsafe { buffer.advance_unchecked(0) };
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.cap, 10);
}

#[should_panic(expected = "internal: set_start out of bounds")]
#[test]
fn test_advance_unchecked_exceed_capacity() {
    let mut buffer = unsafe { BytesMut::with_capacity(5) };
    unsafe { buffer.advance_unchecked(6) };
}

#[test]
fn test_advance_unchecked_valid_case() {
    let mut buffer = unsafe { BytesMut::with_capacity(10) };
    unsafe { buffer.set_len(10) }; // Set length to match capacity
    unsafe { buffer.advance_unchecked(5) };
    assert_eq!(buffer.len(), 5);
    assert_eq!(buffer.cap, 5);
}


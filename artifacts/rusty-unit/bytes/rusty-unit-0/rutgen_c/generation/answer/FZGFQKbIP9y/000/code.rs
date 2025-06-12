// Answer 0

#[test]
fn test_set_len_within_capacity() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.set_len(5);
    }
    assert_eq!(buffer.len(), 5);
    unsafe {
        buffer.set_len(10);
    }
    assert_eq!(buffer.len(), 10);
}

#[test]
#[should_panic(expected = "set_len out of bounds")]
fn test_set_len_exceeding_capacity() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.set_len(11); // This should panic
    }
}

#[test]
fn test_set_len_zero_length() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.set_len(0);
    }
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_set_len_after_initialization() {
    let mut buffer = BytesMut::with_capacity(5);
    unsafe {
        buffer.set_len(5);
    }
    assert_eq!(buffer.len(), 5);
    unsafe {
        buffer.set_len(3);
    }
    assert_eq!(buffer.len(), 3);
}

#[test]
fn test_set_len_on_empty_buffer() {
    let mut buffer = BytesMut::new();
    unsafe {
        buffer.set_len(0);
    }
    assert_eq!(buffer.len(), 0);

    unsafe {
        buffer.set_len(0);
    }
    assert_eq!(buffer.len(), 0);
}


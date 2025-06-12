// Answer 0

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert!(buffer.bytes.len() == i128::MAX_STR_LEN);
}

#[test]
fn test_buffer_contents_initialization() {
    let buffer = Buffer::new();
    for byte in &buffer.bytes {
        assert!(byte.as_ptr().is_null());
    }
}


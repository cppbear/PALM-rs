// Answer 0

#[test]
fn test_clear_non_empty_buffer() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello world"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_empty_buffer() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b""[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_large_buffer() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"hello world, this is a large buffer that we are testing"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn test_clear_buffer_after_modification() {
    use bytes::Bytes;

    let mut buf = Bytes::from(&b"initial buffer"[..]);
    buf.extend_from_slice(&b" more data"[..]);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
#[should_panic] // Note: This test would be just in case internal asserts cause a panic (if it were applicable).
fn test_clear_on_frozen_bytes() {
    use bytes::Bytes;

    let buf = Bytes::from(&b"cannot modify"[..]).freeze(); // Assume freeze makes it immutable.
    let mut panic_buf = buf.clone(); // Clone to test panic
    panic_buf.clear(); // This would panic if frozen.
}


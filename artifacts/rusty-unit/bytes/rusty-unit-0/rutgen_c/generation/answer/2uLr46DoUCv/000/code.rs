// Answer 0

#[test]
fn truncate_within_bounds() {
    let mut buf = Bytes::from_static(&b"hello world"[..]);
    buf.truncate(5);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}

#[test]
fn truncate_to_zero() {
    let mut buf = Bytes::from_static(&b"hello world"[..]);
    buf.truncate(0);
    assert_eq!(buf.len(), 0);
}

#[test]
fn truncate_greater_than_length_no_effect() {
    let mut buf = Bytes::from_static(&b"hello"[..]);
    buf.truncate(10);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}

#[test]
fn truncate_equal_to_length_no_effect() {
    let mut buf = Bytes::from_static(&b"hello"[..]);
    buf.truncate(5);
    assert_eq!(buf.as_slice(), b"hello"[..]);
}


// Answer 0

#[test]
fn test_extend_from_slice_with_exact_capacity() {
    let mut buf = BytesMut::with_capacity(6);
    buf.resize(0, 0);
    buf.extend_from_slice(b"abc");
}

#[test]
fn test_extend_from_slice_with_increased_capacity() {
    let mut buf = BytesMut::with_capacity(2);
    buf.extend_from_slice(b"a");
    buf.extend_from_slice(b"bcd");
}

#[test]
#[should_panic]
fn test_extend_from_slice_with_insufficient_capacity() {
    let mut buf = BytesMut::with_capacity(2);
    buf.extend_from_slice(b"abc");
}

#[test]
fn test_extend_from_slice_with_zero_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"");
}

#[test]
fn test_extend_from_slice_with_large_capacity() {
    let mut buf = BytesMut::with_capacity(100);
    buf.extend_from_slice(b"large byte slice");
}

#[test]
fn test_extend_from_slice_when_empty() {
    let mut buf = BytesMut::new();
    buf.extend_from_slice(b"hello");
}

#[test]
fn test_extend_from_slice_then_empty() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"world");
    buf.clear();
    buf.extend_from_slice(b"test");
}


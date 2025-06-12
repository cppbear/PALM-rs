// Answer 0

#[test]
fn test_put_bytes_with_zero_count() {
    let mut buf = bytes::BufMut::with_capacity(10);
    buf.put_bytes(5, 0);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_put_bytes_with_small_count() {
    let mut buf = bytes::BufMut::with_capacity(10);
    buf.put_bytes(3, 2);
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.get_bytes(0), 3);
    assert_eq!(buf.get_bytes(1), 3);
}

#[test]
fn test_put_bytes_with_boundary_count() {
    let mut buf = bytes::BufMut::with_capacity(usize::MAX);
    let initial_len = buf.len();
    buf.put_bytes(1, usize::MAX - initial_len);
    assert_eq!(buf.len(), usize::MAX);
}

#[test]
#[should_panic]
fn test_put_bytes_with_overflow() {
    let mut buf = bytes::BufMut::with_capacity(usize::MAX);
    buf.put_bytes(1, 1);
}


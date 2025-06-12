// Answer 0

#[test]
fn test_clear_non_empty_buffer() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 5);
}

#[test]
fn test_clear_empty_buffer() {
    let mut buf = BytesMut::new();
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 0);
}

#[test]
fn test_clear_after_resize() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 0);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 10);
}

#[test]
fn test_clear_preserves_capacity() {
    let mut buf = BytesMut::with_capacity(20);
    buf.resize(10, 0);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 20);
}


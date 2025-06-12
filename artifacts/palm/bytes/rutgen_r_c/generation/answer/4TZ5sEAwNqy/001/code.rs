// Answer 0

#[test]
fn test_clear_empty_buffer() {
    let mut buf = BytesMut::new();
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_clear_non_empty_buffer() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 1);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.capacity(), 10);
}

#[test]
fn test_clear_large_buffer() {
    let mut buf = BytesMut::with_capacity(100);
    buf.resize(50, 2);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.capacity(), 100);
}

#[test]
fn test_clear_buf_after_split() {
    let mut buf = BytesMut::with_capacity(20);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    let _other = buf.split_off(3);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}


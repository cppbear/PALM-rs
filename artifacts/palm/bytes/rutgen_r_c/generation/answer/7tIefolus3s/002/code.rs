// Answer 0

#[test]
fn test_put_with_empty_src() {
    let mut buf = Vec::new();
    let empty_buf: Vec<u8> = Vec::new(); // src with no remaining bytes

    buf.put(empty_buf);

    assert!(buf.is_empty());
}

#[test]
fn test_put_with_large_buf() {
    let mut buf = Vec::new();
    let large_buf: Vec<u8> = vec![1; 1024]; // src with 1024 bytes remaining

    buf.put(large_buf);

    assert_eq!(buf.len(), 1024);
    assert_eq!(buf.as_slice(), &[1; 1024]);
}

#[test]
#[should_panic(expected = "capacity overflow")]
fn test_put_with_over_capacity() {
    let mut buf = Vec::with_capacity(2);
    let large_buf: Vec<u8> = vec![1; 3]; // src requesting more than buffer can accommodate

    buf.put(large_buf);
}


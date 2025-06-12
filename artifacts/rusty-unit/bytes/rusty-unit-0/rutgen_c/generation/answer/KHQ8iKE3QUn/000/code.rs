// Answer 0

#[test]
fn test_put_slice_with_empty_slice() {
    let mut buf = Vec::new();
    let src: &[u8] = &[];
    buf.put_slice(src);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_put_slice_with_single_byte() {
    let mut buf = Vec::new();
    let src: &[u8] = &[1];
    buf.put_slice(src);
    assert_eq!(buf.len(), 1);
    assert_eq!(buf[0], 1);
}

#[test]
fn test_put_slice_with_multiple_bytes() {
    let mut buf = Vec::new();
    let src: &[u8] = &[1, 2, 3, 4];
    buf.put_slice(src);
    assert_eq!(buf.len(), 4);
    assert_eq!(buf, vec![1, 2, 3, 4]);
}

#[test]
fn test_put_slice_with_large_slice() {
    let mut buf = Vec::new();
    let src: &[u8] = &[0; 1024];
    buf.put_slice(src);
    assert_eq!(buf.len(), 1024);
}

#[test]
fn test_put_slice_with_slice_exceeding_capacity() {
    let mut buf = Vec::with_capacity(5);
    let src: &[u8] = &[1, 2, 3, 4, 5, 6];
    buf.put_slice(src);
    assert_eq!(buf.len(), 6);
    assert_eq!(buf, vec![1, 2, 3, 4, 5, 6]);
}


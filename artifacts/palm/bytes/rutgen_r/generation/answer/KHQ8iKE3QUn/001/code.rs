// Answer 0

#[test]
fn test_put_slice_empty_slice() {
    let mut buf = bytes::BufMut::with_capacity(10);
    buf.put_slice(&[]);
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_put_slice_non_empty_slice() {
    let mut buf = bytes::BufMut::with_capacity(10);
    buf.put_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_put_slice_large_slice() {
    let mut buf = bytes::BufMut::with_capacity(10);
    buf.put_slice(&[1; 15]); // filling with 15 bytes of 1
    assert_eq!(buf.len(), 15);
    assert_eq!(buf.as_slice(), &[1; 15]);
}

#[test]
fn test_put_slice_boundary_capacity() {
    let mut buf = bytes::BufMut::with_capacity(5);
    buf.put_slice(&[1, 2, 3, 4, 5]); // exactly fills the buffer
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
}

#[should_panic]
fn test_put_slice_exceeding_capacity() {
    let mut buf = bytes::BufMut::with_capacity(5);
    buf.put_slice(&[1, 2, 3, 4, 5, 6]); // exceeds the capacity
}


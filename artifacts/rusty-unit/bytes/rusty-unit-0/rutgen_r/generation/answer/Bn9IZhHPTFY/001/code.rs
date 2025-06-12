// Answer 0

#[test]
fn test_put_slice_empty_slice() {
    let mut bytes = bytes::BytesMut::new();
    let src: &[u8] = &[];
    bytes.put_slice(src);
    assert_eq!(bytes.len(), 0);
}

#[test]
fn test_put_slice_non_empty_slice() {
    let mut bytes = bytes::BytesMut::with_capacity(10);
    let src: &[u8] = &[1, 2, 3, 4, 5];
    bytes.put_slice(src);
    assert_eq!(bytes.len(), 5);
    assert_eq!(&bytes[..], src);
}

#[test]
fn test_put_slice_exceeding_capacity() {
    let mut bytes = bytes::BytesMut::with_capacity(5);
    let src: &[u8] = &[6, 7, 8, 9, 10, 11];
    bytes.put_slice(src);
    assert_eq!(bytes.len(), 6);
    assert_eq!(&bytes[5..], src);
}

#[test]
#[should_panic]
fn test_put_slice_null_pointer() {
    let mut bytes = bytes::BytesMut::new();
    let src: &[u8] = std::ptr::null();
    bytes.put_slice(src);
} 

#[test]
fn test_put_slice_large_slice() {
    let mut bytes = bytes::BytesMut::new();
    let src: Vec<u8> = (0..1000).map(|x| x as u8).collect();
    bytes.put_slice(&src);
    assert_eq!(bytes.len(), 1000);
    assert_eq!(&bytes[..], &src[..]);
}


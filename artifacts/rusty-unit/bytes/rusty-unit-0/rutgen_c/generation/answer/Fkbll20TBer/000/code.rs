// Answer 0

#[test]
fn test_from_static() {
    let byte_slice: &'static [u8] = b"hello";
    let bytes = Bytes::from_static(byte_slice);
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, byte_slice);
}

#[test]
fn test_from_static_empty() {
    let byte_slice: &'static [u8] = b"";
    let bytes = Bytes::from_static(byte_slice);
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, byte_slice);
    assert_eq!(bytes.len, 0);
}

#[test]
fn test_from_static_single_element() {
    let byte_slice: &'static [u8] = b"a";
    let bytes = Bytes::from_static(byte_slice);
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, byte_slice);
    assert_eq!(bytes.len, 1);
}


// Answer 0

#[test]
fn test_copy_from_slice_non_empty() {
    let data = &[1, 2, 3, 4, 5];
    let bytes = Bytes::copy_from_slice(data);
    assert_eq!(bytes.len(), data.len());
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, data);
}

#[test]
fn test_copy_from_slice_empty() {
    let data: &[u8] = &[];
    let bytes = Bytes::copy_from_slice(data);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.is_empty());
}

#[test]
fn test_copy_from_slice_boundary() {
    let data = &[0, 255];
    let bytes = Bytes::copy_from_slice(data);
    assert_eq!(bytes.len(), data.len());
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, data);
}

#[should_panic(expected = "slice must not be empty")]
#[test]
fn test_copy_from_slice_panic() {
    let data: &[u8] = &[];
    let _ = Bytes::copy_from_slice(data);
}


// Answer 0

#[test]
fn test_bytes_mut_new() {
    let bytes = BytesMut::new();
    assert_eq!(0, bytes.len());
    assert_eq!(0, bytes.capacity());
}

#[test]
fn test_bytes_mut_new_non_empty() {
    let mut bytes = BytesMut::new();
    bytes.reserve(2);
    bytes.extend_from_slice(b"xy");
    assert_eq!(2, bytes.len());
    assert_eq!(2, bytes.capacity());
    assert_eq!(&b"xy"[..], bytes.as_slice());
}


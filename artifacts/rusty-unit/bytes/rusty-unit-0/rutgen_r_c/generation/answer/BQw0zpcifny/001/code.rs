// Answer 0

#[test]
fn test_new_empty_bytes() {
    let b = Bytes::new();
    assert_eq!(b.len(), 0);
    assert!(b.is_empty());
    assert_eq!(&b[..], b"");
}

#[test]
fn test_new_bytes_not_panic() {
    let b = Bytes::new();
    let empty_slice: &[u8] = &[];
    assert_eq!(b.slice(..), Bytes::from_static(empty_slice));
}

#[test]
fn test_new_bytes_slice_ref() {
    let b = Bytes::new();
    let slice_ref_result = b.slice_ref(b"");
    assert!(slice_ref_result.is_empty());
}


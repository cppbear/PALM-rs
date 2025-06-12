// Answer 0

#[test]
fn test_is_empty_with_empty_bytes() {
    let b = Bytes::new();
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_bytes() {
    let b = Bytes::copy_from_slice(&[1, 2, 3]);
    assert!(!b.is_empty());
}


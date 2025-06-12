// Answer 0

#[test]
fn test_borrow_non_empty() {
    let bytes = Bytes::from_static(b"hello");
    let slice = bytes.borrow();
    assert_eq!(slice, b"hello");
}

#[test]
fn test_borrow_empty() {
    let bytes = Bytes::new();
    let slice = bytes.borrow();
    assert_eq!(slice, b"");
}


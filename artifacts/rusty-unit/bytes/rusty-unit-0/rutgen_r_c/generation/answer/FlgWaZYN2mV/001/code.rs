// Answer 0

#[test]
fn test_split_off_at_len() {
    let mut bytes = Bytes::from_static(b"hello world");

    let result = bytes.split_off(bytes.len());

    assert_eq!(bytes.len(), 0);
    assert_eq!(result.len(), 11);
    assert_eq!(&result.as_slice(), b"hello world");
}

#[test]
fn test_split_off_at_zero() {
    let mut bytes = Bytes::from_static(b"hello world");

    let result = bytes.split_off(0);

    assert_eq!(bytes.len(), 11);
    assert_eq!(result.len(), 0);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::from_static(b"hello world");
    
    // This should panic since at is greater than len()
    let _ = bytes.split_off(12);
}


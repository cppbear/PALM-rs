// Answer 0

#[test]
fn test_split_to_empty_bytes() {
    let mut bytes = Bytes::new();
    let result = bytes.split_to(0);
    assert_eq!(bytes.len(), 0);
    assert_eq!(result.len(), 0);
}

#[test]
#[should_panic]
fn test_split_to_panic_out_of_bounds() {
    let mut bytes = Bytes::from_static(b"hello");
    bytes.split_to(6); // This should panic since 6 > len
}

#[test]
fn test_split_to_at_zero() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(0);
    assert_eq!(bytes.len(), 5);
    assert_eq!(result.len(), 0);
    assert_eq!(&bytes.as_slice()[..], b"hello");
}

#[test]
fn test_split_to_halfway() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_to(5);
    assert_eq!(bytes.len(), 6);
    assert_eq!(result.len(), 5);
    assert_eq!(&bytes.as_slice()[..], b" world");
    assert_eq!(&result.as_slice()[..], b"hello");
}


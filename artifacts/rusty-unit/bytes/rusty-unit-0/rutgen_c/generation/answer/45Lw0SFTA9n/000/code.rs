// Answer 0

#[test]
fn test_split_to_empty() {
    let mut bytes = Bytes::new();
    let result = bytes.split_to(0);
    assert_eq!(bytes.len(), 0);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_split_to_full() {
    let bytes = Bytes::from_static(b"hello");
    let mut bytes_mut = bytes.clone(); // mutable clone for splitting
    let result = bytes_mut.split_to(5);
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(&result.as_slice(), b"hello");
}

#[test]
#[should_panic(expected = "split_to out of bounds")]
fn test_split_to_out_of_bounds() {
    let bytes = Bytes::from_static(b"hello");
    let mut bytes_mut = bytes.clone(); // mutable clone for splitting
    bytes_mut.split_to(6); // this should panic
}

#[test]
fn test_split_to_non_zero() {
    let bytes = Bytes::from_static(b"hello world");
    let mut bytes_mut = bytes.clone(); // mutable clone for splitting
    let result = bytes_mut.split_to(5);
    assert_eq!(bytes_mut.len(), 6);
    assert_eq!(&result.as_slice(), b"hello");
}

#[test]
fn test_split_to_zero() {
    let bytes = Bytes::from_static(b"hello");
    let mut bytes_mut = bytes.clone(); // mutable clone for splitting
    let result = bytes_mut.split_to(0);
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(result.len(), 0);
}


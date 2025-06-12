// Answer 0

#[test]
fn test_split_to_non_zero_index() {
    let mut a = Bytes::from_static(b"hello world");
    let b = a.split_to(5);
    
    assert_eq!(a.as_slice(), b" world");
    assert_eq!(b.as_slice(), b"hello");
}

#[test]
fn test_split_to_edge_case() {
    let mut a = Bytes::from_static(b"example");
    let b = a.split_to(3);

    assert_eq!(a.as_slice(), b"ple");
    assert_eq!(b.as_slice(), b"ex");
}

#[test]
#[should_panic]
fn test_split_to_panic_at_greater_than_length() {
    let mut a = Bytes::from_static(b"test");
    let _ = a.split_to(5);  // This should panic
}

#[test]
fn test_split_to_empty_string() {
    let mut a = Bytes::from_static(b"");
    let b = a.split_to(0);

    assert_eq!(a.as_slice(), b"");
    assert_eq!(b.len(), 0);
}


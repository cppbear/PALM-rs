// Answer 0

#[test]
fn test_split_to_non_empty() {
    let mut a = Bytes::copy_from_slice(b"hello world");
    let b = a.split_to(5);

    assert_eq!(&a.as_slice()[..], b" world");
    assert_eq!(&b.as_slice()[..], b"hello");
}

#[test]
fn test_split_to_at_length() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let b = a.split_to(5); // at == self.len()

    assert_eq!(&a.as_slice()[..], b"");
    assert_eq!(&b.as_slice()[..], b"hello");
}

#[test]
fn test_split_to_zero() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let b = a.split_to(0); // at == 0

    assert_eq!(&a.as_slice()[..], b"hello");
    assert_eq!(&b.as_slice()[..], b"");
}

#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let _ = a.split_to(6); // at > self.len(), should panic
}


// Answer 0

#[test]
fn test_split_off_zero_at() {
    let mut a = Bytes::copy_from_slice(b"hello");
    let b = a.split_off(0);

    assert_eq!(a.len(), 0);
    assert_eq!(b.len(), 5);
    assert_eq!(&b.as_slice()[..], b"hello");
}

#[test]
fn test_split_off_at_len() {
    let mut a = Bytes::copy_from_slice(b"world");
    let b = a.split_off(5); // at == self.len()

    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 0);
}

#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut a = Bytes::copy_from_slice(b"hello");
    a.split_off(6); // at > self.len()
}

#[test]
fn test_split_off_non_empty() {
    let mut a = Bytes::copy_from_slice(b"hello world");
    let b = a.split_off(5);

    assert_eq!(a.len(), 5);
    assert_eq!(b.len(), 6);
    assert_eq!(&a.as_slice()[..], b"hello");
    assert_eq!(&b.as_slice()[..], b" world");
}


// Answer 0

#[test]
fn test_split_to_boundary_case() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let other = buf.split_to(buf.len());

    assert_eq!(buf.len(), 0);
    assert_eq!(other.len(), 5);
    assert_eq!(other.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
#[should_panic(expected = "split_to out of bounds")]
fn test_split_to_panic_case() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    buf.split_to(6);
}

#[test]
fn test_split_to_non_empty() {
    let mut buf = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let other = buf.split_to(3);

    assert_eq!(buf.len(), 2);
    assert_eq!(other.len(), 3);
    assert_eq!(buf.as_slice(), &[4, 5]);
    assert_eq!(other.as_slice(), &[1, 2, 3]);
}


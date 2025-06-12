// Answer 0

#[test]
fn test_split_off_within_bounds() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let other = bytes_mut.split_off(2);

    assert_eq!(bytes_mut.len(), 2);
    assert_eq!(other.len(), 3);
    assert_eq!(&*bytes_mut.as_slice(), &[1, 2]);
    assert_eq!(&*other.as_slice(), &[3, 4, 5]);
}

#[test]
#[should_panic(expected = "split_off out of bounds")]
fn test_split_off_out_of_bounds() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    bytes_mut.split_off(6);
}

#[test]
fn test_split_off_zero() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let other = bytes_mut.split_off(0);

    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(other.len(), 5);
    assert_eq!(&*bytes_mut.as_slice(), &[]);
    assert_eq!(&*other.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_split_off_empty() {
    let mut bytes_mut = BytesMut::new();
    let other = bytes_mut.split_off(0);

    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(other.len(), 0);
}


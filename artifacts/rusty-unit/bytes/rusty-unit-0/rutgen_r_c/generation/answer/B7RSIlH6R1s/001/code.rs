// Answer 0

#[test]
fn test_split_off_with_capacity() {
    let mut bytes = BytesMut::with_capacity(10);
    bytes.resize(10, 0); // Fill the BytesMut to its capacity
    let other = bytes.split_off(10); // Split at capacity
    assert_eq!(bytes.len(), 10);
    assert_eq!(other.len(), 0);
    assert_eq!(bytes.capacity(), 10);
}

#[test]
#[should_panic(expected = "split_off out of bounds")]
fn test_split_off_panic_out_of_bounds() {
    let mut bytes = BytesMut::with_capacity(10);
    bytes.resize(10, 0);
    bytes.split_off(11); // Attempt to split beyond the capacity
}

#[test]
fn test_split_off_correct_values() {
    let mut bytes = BytesMut::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let other = bytes.split_off(5);
    assert_eq!(&bytes[..], [1, 2, 3, 4, 5]);
    assert_eq!(&other[..], [6, 7, 8, 9, 10]);
    assert_eq!(bytes.len(), 5);
    assert_eq!(other.len(), 5);
}


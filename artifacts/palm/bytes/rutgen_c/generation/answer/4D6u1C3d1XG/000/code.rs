// Answer 0

#[test]
fn test_split_to_within_bounds() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let split_bytes = bytes_mut.split_to(3);

    assert_eq!(bytes_mut.len(), 2);
    assert_eq!(split_bytes.len(), 3);
    assert_eq!(bytes_mut.as_slice(), &[4, 5]);
    assert_eq!(split_bytes.as_slice(), &[1, 2, 3]);
}

#[test]
#[should_panic(expected = "split_to out of bounds: 5 <= 5")]
fn test_split_to_out_of_bounds() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3]);
    bytes_mut.split_to(5);
}

#[test]
fn test_split_to_empty() {
    let mut bytes_mut = BytesMut::new();
    let split_bytes = bytes_mut.split_to(0);

    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(split_bytes.len(), 0);
}


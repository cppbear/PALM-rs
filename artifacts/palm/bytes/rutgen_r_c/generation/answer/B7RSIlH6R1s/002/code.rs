// Answer 0

#[test]
#[should_panic(expected = "split_off out of bounds")]
fn test_split_off_bounds_panics() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Attempt to split off at an index greater than the capacity.
    let _ = bytes_mut.split_off(11);
}

#[test]
fn test_split_off_valid_case() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let split_point = 5;
    let other = bytes_mut.split_off(split_point);

    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(other.len(), 5);
    assert_eq!(&bytes_mut.as_slice()[..], &[1, 2, 3, 4, 5]);
    assert_eq!(&other.as_slice()[..], &[6, 7, 8, 9, 10]);
}

#[test]
fn test_split_off_boundary_case_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4]);

    let other = bytes_mut.split_off(0);

    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(other.len(), 4);
    assert_eq!(&other.as_slice()[..], &[1, 2, 3, 4]);
}

#[test]
fn test_split_off_capacity_equal() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);

    let other = bytes_mut.split_off(5);

    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(other.len(), 0);
}


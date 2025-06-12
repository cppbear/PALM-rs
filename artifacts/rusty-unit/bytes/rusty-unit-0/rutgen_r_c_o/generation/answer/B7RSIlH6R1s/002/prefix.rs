// Answer 0

#[test]
fn test_split_off_panic_at_greater_than_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    // Setting up BytesMut to have a capacity of 5
    byte_mut.resize(5, 0);
    let capacity = bytes_mut.capacity();
    // Attempt to split off at an index greater than capacity
    let _ = bytes_mut.split_off(capacity + 1);
}

#[test]
fn test_split_off_panic_at_equal_to_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(2);
    // Setting up BytesMut to have a capacity of 2
    bytes_mut.resize(2, 0);
    let capacity = bytes_mut.capacity();
    // Attempt to split off at an index equal to capacity
    let _ = bytes_mut.split_off(capacity);
}

#[test]
fn test_split_off_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    // Setting up BytesMut to have a capacity of 10 and a length of 10
    bytes_mut.resize(10, 0);
    // Split off at index 0
    let result = bytes_mut.split_off(0);
}

#[test]
fn test_split_off_middle() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    // Setting up BytesMut to have a capacity of 10 and a length of 10
    bytes_mut.resize(10, 0);
    // Split off at index 5
    let result = bytes_mut.split_off(5);
}

#[test]
fn test_split_off_capacity_zero() {
    let mut bytes_mut = BytesMut::new();
    // Attempting to split off an empty BytesMut
    let result = bytes_mut.split_off(0);
}


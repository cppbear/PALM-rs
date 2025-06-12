// Answer 0

#[test]
#[should_panic]
fn test_split_to_panic_at_negative() {
    let mut bytes_mut = BytesMut::new();
    let at: usize = usize::MAX; // This will cause panics due to at being greater than len
    bytes_mut.split_to(at);
}

#[test]
#[should_panic]
fn test_split_to_panic_at_greater_than_len() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.resize(5, 0); // Now len is 5
    let at: usize = 6; // This will cause panics as at is greater than len
    bytes_mut.split_to(at);
}

#[test]
fn test_split_to_boundaries() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 1); // Now len is 10
    let at: usize = 10; // This is equal to the length and should work
    let result = bytes_mut.split_to(at);
    // Resulting BytesMut should have len 10 and capacity >= 10
    let expected_len = 10;
}

#[test]
fn test_split_to_empty() {
    let mut bytes_mut = BytesMut::new(); // len is 0
    let at: usize = 0; // This should be fine
    let result = bytes_mut.split_to(at);
    // Resulting BytesMut should have len 0
    let expected_len = 0;
}


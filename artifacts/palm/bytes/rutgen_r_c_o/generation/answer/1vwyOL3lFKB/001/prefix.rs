// Answer 0

#[test]
fn test_advance_with_zero_cnt() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.resize(10, 0); // Ensure capacity is enough
    bytes_mut.advance(0);
}

#[test]
fn test_advance_with_remaining_count() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.resize(5, 1); // Set length to capacity
    bytes_mut.advance(bytes_mut.remaining());
}

#[test]
fn test_advance_with_max_count() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    bytes_mut.resize(usize::MAX, 0); // Set length to maximum
    bytes_mut.advance(bytes_mut.remaining());
}

#[test]
fn test_advance_with_small_non_zero_count() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 1); // Set length to 5
    bytes_mut.advance(3); // Advance by 3
}

#[test]
fn test_advance_with_all_available() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    bytes_mut.resize(15, 1); // Set length to 15
    bytes_mut.advance(15); // Advance by all
}

#[should_panic]
fn test_advance_panic_too_much() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 1); // Set length to 5
    bytes_mut.advance(6); // Trigger panic
}


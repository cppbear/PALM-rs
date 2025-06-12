// Answer 0

#[test]
#[should_panic]
fn test_advance_panic_over_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5
    let cnt = 6; // cnt is greater than remaining (5)
    bytes_mut.advance(cnt);
}

#[test]
#[should_panic]
fn test_advance_panic_exceeding_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5
    let cnt = 11; // cnt is greater than capacity
    bytes_mut.advance(cnt);
}

#[test]
fn test_advance_exact_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5
    let cnt = 5; // cnt equals remaining
    bytes_mut.advance(cnt);
}

#[test]
fn test_advance_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5
    let cnt = 0; // cnt is zero
    bytes_mut.advance(cnt);
}

#[test]
fn test_advance_one() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5
    let cnt = 1; // cnt is less than remaining
    bytes_mut.advance(cnt);
}

#[test]
#[should_panic]
fn test_advance_one_past_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set len to 5
    let cnt = 6; // cnt is one more than remaining
    bytes_mut.advance(cnt);
}


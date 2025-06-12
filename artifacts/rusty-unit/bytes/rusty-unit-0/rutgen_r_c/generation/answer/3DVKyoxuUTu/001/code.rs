// Answer 0

#[test]
fn test_advance_at_length() {
    let mut bytes = Bytes::from_static(b"hello");

    // cnt equals the current length of the Bytes instance.
    let cnt = bytes.len();
    bytes.advance(cnt); // Should not panic
    assert!(bytes.len() == 0); // After advancing, length should be 0
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`")]
fn test_advance_over_length() {
    let mut bytes = Bytes::from_static(b"hello");

    // cnt exceeds the current length of the Bytes instance.
    let cnt = bytes.len() + 1;
    bytes.advance(cnt); // Should panic
}

#[test]
fn test_advance_zero() {
    let mut bytes = Bytes::from_static(b"hello");

    // cnt is zero.
    let cnt = 0;
    bytes.advance(cnt); // Should not panic
    assert!(bytes.len() == 5); // Length should remain the same
}


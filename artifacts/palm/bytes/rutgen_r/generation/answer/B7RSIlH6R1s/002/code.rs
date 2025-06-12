// Answer 0

#[test]
#[should_panic(expected = "split_off out of bounds:")]
fn test_split_off_panic_exceeds_capacity() {
    use bytes::BytesMut;

    // Initialize a BytesMut with a known capacity
    let mut bytes = BytesMut::from(&b"hello world"[..]);

    // Attempt to split off at an index greater than the capacity (which is 11)
    let _result = bytes.split_off(12);
}


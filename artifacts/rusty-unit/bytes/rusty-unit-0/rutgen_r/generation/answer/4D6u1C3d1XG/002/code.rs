// Answer 0

#[test]
#[should_panic(expected = "split_to out of bounds")]
fn test_split_to_panic_out_of_bounds() {
    use bytes::BytesMut;

    let mut a = BytesMut::from(&b"hello"[..]);
    // Trying to split at an index greater than the length
    let _b = a.split_to(6);
}

#[test]
#[should_panic(expected = "split_to out of bounds")]
fn test_split_to_panic_empty() {
    use bytes::BytesMut;

    let mut a = BytesMut::new();
    // Trying to split an empty BytesMut
    let _b = a.split_to(1);
}


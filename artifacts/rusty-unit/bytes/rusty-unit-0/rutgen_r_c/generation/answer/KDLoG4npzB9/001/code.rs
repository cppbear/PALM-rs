// Answer 0

#[test]
fn test_split_off_must_use_valid() {
    use bytes::BytesMut;

    // Test valid split_off usage
    let mut b1 = BytesMut::from("hello world");
    let b2 = b1.split_off(6);
    assert_eq!(b2, BytesMut::from("world"));
}

#[test]
#[should_panic]
fn test_split_off_panic_too_large() {
    use bytes::BytesMut;

    // Testing split_off with a position out of bounds
    let mut b1 = BytesMut::from("hello world");
    let _ = b1.split_off(20); // This should panic
}

#[test]
#[should_panic]
fn test_split_off_panic_negative() {
    use bytes::BytesMut;

    // Testing split_off with a negative position
    let mut b1 = BytesMut::from("hello world");
    let _ = b1.split_off(usize::MAX); // This should panic as well
}

#[test]
fn test_split_off_boundary() {
    use bytes::BytesMut;

    // Test boundary condition - splitting at start
    let mut b1 = BytesMut::from("hello world");
    let b2 = b1.split_off(0);
    assert_eq!(b2, BytesMut::from("hello world")); // Should return the whole buffer

    // Test boundary condition - splitting at end
    let mut b1 = BytesMut::from("hello world");
    let b2 = b1.split_off(11);
    assert_eq!(b2, BytesMut::from("")); // Should return an empty buffer
}

#[test]
fn test_split_off_empty() {
    use bytes::BytesMut;

    // Test splitting an empty buffer
    let mut b1 = BytesMut::from("");
    let b2 = b1.split_off(0);
    assert_eq!(b2, BytesMut::from("")); // Should return an empty buffer
}


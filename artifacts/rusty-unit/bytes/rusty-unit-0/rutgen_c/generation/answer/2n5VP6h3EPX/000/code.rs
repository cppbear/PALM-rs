// Answer 0

#[test]
fn test_split_on_non_empty_bytesmut() {
    // Initialize a BytesMut with some capacity and data
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(&b"hello world"[..]);

    // Call the split function
    let other = buf.split();

    // Check that the original buf is now empty
    assert!(buf.is_empty());
    // Check that the capacity is unchanged
    assert_eq!(buf.capacity(), 1024);
    // Check that the split buffer contains the original data
    assert_eq!(other.as_slice(), b"hello world"[..]);
}

#[test]
fn test_split_on_empty_bytesmut() {
    // Initialize a BytesMut with no data
    let mut buf = BytesMut::new();

    // Call the split function
    let other = buf.split();

    // Check that the original buf is empty
    assert!(buf.is_empty());
    // Check that the capacity remains 0
    assert_eq!(buf.capacity(), 0);
    // Check that the split buffer is also empty
    assert!(other.is_empty());
}


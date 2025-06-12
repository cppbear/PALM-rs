// Answer 0

#[test]
fn test_resize_increases_length() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    
    // Initially resizing with a new_len greater than the original length
    buf.resize(3, 0x1);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);

    // Further increasing the length
    buf.resize(5, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x2, 0x2, 0x2]);
}

#[test]
fn test_resize_decreases_length() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.resize(4, 0x1); // Start with length 4
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1, 0x1]);

    // Now decrease size
    buf.resize(2, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x1]);
}

#[test]
fn test_resize_no_change() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);

    // Resize to the same size - should not change the buffer
    buf.resize(3, 0x2);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);
}

#[test]
fn test_resize_to_zero() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    buf.resize(3, 0x1);
    assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);

    // Resize to zero length
    buf.resize(0, 0x2);
    assert_eq!(&buf[..], &[]);
}

#[test]
fn test_resize_edge_case() {
    use bytes::BytesMut;

    let mut buf = BytesMut::new();
    
    // Start small and increase
    buf.resize(1, 0x1);
    assert_eq!(&buf[..], &[0x1]);

    // Resize to the same length - should do nothing
    buf.resize(1, 0x2);
    assert_eq!(&buf[..], &[0x1]);
}


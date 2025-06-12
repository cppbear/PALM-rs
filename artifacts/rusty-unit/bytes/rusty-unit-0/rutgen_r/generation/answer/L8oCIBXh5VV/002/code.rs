// Answer 0

#[test]
fn test_put_bytes_exact_capacity() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 6); // Fill the buffer to its exact capacity

        assert_eq!(0, buf.remaining_mut()); // No remaining capacity
    }
    assert_eq!(b"aaaaaa", &dst); // Verify buffer content
}

#[test]
fn test_put_bytes_partial_fill() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'b', 4); // Fill part of the buffer

        assert_eq!(2, buf.remaining_mut()); // Check remaining capacity
    }
    assert_eq!(b"bbbb\0\0", &dst); // Verify buffer content
}

#[should_panic]
fn test_put_bytes_insufficient_capacity() {
    let mut dst = [0; 3];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'c', 4); // This should panic due to insufficient capacity
    }
}

#[test]
fn test_put_bytes_zero_count() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'd', 0); // Call with cnt = 0, should not alter the buffer

        assert_eq!(6, buf.remaining_mut()); // Check all capacity remains
    }
    assert_eq!(b"\0\0\0\0\0\0", &dst); // Verify that buffer content is unchanged
}


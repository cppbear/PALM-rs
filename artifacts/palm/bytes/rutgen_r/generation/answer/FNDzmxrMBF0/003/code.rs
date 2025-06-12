// Answer 0


#[test]
fn test_put_slice_full_capacity() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"hello");
        
        assert_eq!(1, buf.remaining_mut());
    }
    assert_eq!(b"hello\0", &dst);
}

#[test]
#[should_panic]
fn test_put_slice_exceed_capacity() {
    let mut dst = [0; 5];
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"hello world"); // Exceeds capacity
        
        // This line will not be reached due to panic
        assert_eq!(buf.remaining_mut(), 0);
    }
}

#[test]
fn test_put_slice_empty_source() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"");
        
        assert_eq!(6, buf.remaining_mut()); // Remaining capacity should remain unchanged
    }
    assert_eq!(b"\0\0\0\0\0\0", &dst); // No changes to dst
}

#[test]
#[should_panic]
fn test_put_slice_partial_overflow() {
    let mut dst = [0; 10];
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"hello");
        
        // This assumes we will try to copy partially beyond the range
        // Actual panic will occur in `dst[..cnt].copy_from_slice(&src[..cnt]);`
        buf.put_slice(b"hello world"); // Attempt to write out of bounds
    }
}



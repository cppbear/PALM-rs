// Answer 0

#[test]
fn test_put_bytes_with_exact_capacity() {
    let mut dst = [0; 6];
    
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 4);
        
        assert_eq!(2, buf.remaining_mut());
    }
    
    assert_eq!(b"aaaa\0\0", &dst);
}

#[test]
#[should_panic(expected = "requested")]
fn test_put_bytes_with_insufficient_capacity() {
    let mut dst = [0; 6];

    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 7); // This should panic
    }
}

#[test]
fn test_put_bytes_with_remaining_zero() {
    let mut dst = [0; 6];
    
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 0); // Should not panic
        assert_eq!(6, buf.remaining_mut());
    }

    assert_eq!(b"\0\0\0\0\0\0", &dst);
}

#[test]
fn test_put_bytes_with_partial_chunk() {
    let mut dst = [0; 6];

    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 5);
        
        assert_eq!(1, buf.remaining_mut());
    }
    
    assert_eq!(b"aaaaa\0", &dst);
}


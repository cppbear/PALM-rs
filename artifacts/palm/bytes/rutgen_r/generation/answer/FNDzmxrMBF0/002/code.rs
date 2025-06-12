// Answer 0

#[test]
fn test_put_slice_with_exact_capacity() {
    let mut dst = [0; 5];
    
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"hello");
        
        assert_eq!(1, buf.remaining_mut());
    }

    assert_eq!(b"hello\0", &dst);
}

#[test]
fn test_put_slice_with_empty_src() {
    let mut dst = [0; 5];

    {
        let mut buf = &mut dst[..];
        buf.put_slice(&[]);
        
        assert_eq!(5, buf.remaining_mut());
    }

    assert_eq!(b"\0\0\0\0\0", &dst);
}

#[should_panic(expected = "TryGetError")]
fn test_put_slice_with_insufficient_capacity() {
    let mut dst = [0; 5];

    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"hello world"); // This should panic
    }
}


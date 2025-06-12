// Answer 0

#[test]
fn test_put_slice_success() {
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
fn test_put_slice_overflow() {
    let mut dst = [0; 5];
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"hello world");
    }
}

#[test]
fn test_put_slice_empty_src() {
    let mut dst = [0; 5];
    {
        let mut buf = &mut dst[..];
        buf.put_slice(b"");
        assert_eq!(0, buf.remaining_mut());
    }
    assert_eq!(b"\0\0\0\0\0", &dst);
}


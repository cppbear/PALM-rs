// Answer 0

#[test]
fn test_put_bytes_success() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 6);
        assert_eq!(buf.remaining_mut(), 0);
    }
    assert_eq!(dst, [b'a'; 6]);
}

#[test]
#[should_panic]
fn test_put_bytes_panic_not_enough_capacity() {
    let mut dst = [0; 4];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 6);
    }
}

#[test]
fn test_put_bytes_zero_count() {
    let mut dst = [0; 5];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 0);
        assert_eq!(buf.remaining_mut(), 5);
    }
    assert_eq!(dst, [0; 5]);
}


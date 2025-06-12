// Answer 0

#[test]
fn test_put_bytes_with_remaining_capacity_equals_cnt() {
    let mut dst = [0; 6];

    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 6);

        assert_eq!(0, buf.remaining_mut());
    }

    assert_eq!(b"aaaaaa", &dst);
}

#[test]
fn test_put_bytes_with_cnt_zero() {
    let mut dst = [0; 6];

    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 0);

        assert_eq!(6, buf.remaining_mut());
    }

    assert_eq!(b"\0\0\0\0\0\0", &dst);
}

#[should_panic]
fn test_put_bytes_with_not_enough_capacity() {
    let mut dst = [0; 3];

    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 4);
    }
}


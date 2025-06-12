// Answer 0

#[test]
fn test_put_bytes_success() {
    let mut dst = [0; 6];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst[..].map(core::mem::MaybeUninit::new);
        buf.put_bytes(b'a', 6);
        assert_eq!(2, buf.remaining_mut()); // remaining_mut should be 2 (after writing 4 bytes)
    }
    assert_eq!(b"aaaaaa", &dst); // Verify that the bytes have been written correctly
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 0 but advancing by 1")]
fn test_put_bytes_not_enough_capacity() {
    let mut dst = [0; 4];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst[..].map(core::mem::MaybeUninit::new);
        buf.put_bytes(b'a', 5); // This should panic since remaining is 4
    }
}

#[test]
fn test_put_bytes_zero_count() {
    let mut dst = [0; 6];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut dst[..].map(core::mem::MaybeUninit::new);
        buf.put_bytes(b'a', 0); // Should not panic and do nothing
        assert_eq!(4, buf.remaining_mut()); // remaining should still be 4 (unchanged)
    }
    assert_eq!(b"\0\0\0\0\0\0", &dst); // Verify that the buffer is unchanged
}


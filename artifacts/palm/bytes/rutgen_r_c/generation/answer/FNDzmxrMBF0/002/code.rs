// Answer 0

#[test]
fn test_put_slice_fills_exact_capacity() {
    let mut dst = [0; 6];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = unsafe {
            core::slice::from_raw_parts_mut(dst.as_mut_ptr().cast(), dst.len())
        };
        buf.put_slice(b"hello");

        assert_eq!(1, buf.remaining_mut());
    }
    assert_eq!(b"hello\0", &dst);
}

#[test]
#[should_panic]
fn test_put_slice_panics_on_insufficient_capacity() {
    let mut dst = [0; 5];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = unsafe {
            core::slice::from_raw_parts_mut(dst.as_mut_ptr().cast(), dst.len())
        };
        buf.put_slice(b"hello world");
    }
}

#[test]
fn test_put_slice_with_empty_src() {
    let mut dst = [0; 6];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = unsafe {
            core::slice::from_raw_parts_mut(dst.as_mut_ptr().cast(), dst.len())
        };
        buf.put_slice(&[]);

        assert_eq!(6, buf.remaining_mut());
    }
    assert_eq!(b"\0\0\0\0\0\0", &dst);
}


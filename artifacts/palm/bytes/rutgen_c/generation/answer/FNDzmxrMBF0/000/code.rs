// Answer 0

#[test]
fn test_put_slice() {
    let mut dst = [0u8; 6];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = unsafe {
            &mut *(dst.as_mut_ptr() as *mut _)
        };
        buf.put_slice(b"hello");

        assert_eq!(1, buf.remaining_mut());
    }
    assert_eq!(b"hello\0", &dst);
}

#[test]
#[should_panic]
fn test_put_slice_not_enough_capacity() {
    let mut dst = [0u8; 5];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = unsafe {
            &mut *(dst.as_mut_ptr() as *mut _)
        };
        buf.put_slice(b"hello world"); // Exceeds available capacity
    }
}

#[test]
fn test_put_slice_partial() {
    let mut dst = [0u8; 10];
    {
        let mut buf: &mut [core::mem::MaybeUninit<u8>] = unsafe {
            &mut *(dst.as_mut_ptr() as *mut _)
        };
        buf.put_slice(b"hello");
        
        assert_eq!(5, buf.remaining_mut());
        buf.put_slice(b"123");
    }
    assert_eq!(b"hello123\0\0\0", &dst);
}


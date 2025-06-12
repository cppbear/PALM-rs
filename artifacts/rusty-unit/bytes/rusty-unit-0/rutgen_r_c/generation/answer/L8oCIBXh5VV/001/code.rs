// Answer 0

#[test]
#[should_panic]
fn test_put_bytes_panics_when_not_enough_capacity() {
    let mut buf = &mut [core::mem::MaybeUninit::<u8>::uninit(); 3];
    unsafe {
        let buf_mut: &mut [u8] = core::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len());
        buf_mut.put_bytes(b'a', 5);
    }
}

#[test]
fn test_put_bytes_fills_buffer_correctly() {
    let mut dst = [core::mem::MaybeUninit::<u8>::uninit(); 6];
    unsafe {
        let buf = &mut UninitSlice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u8, dst.len());
        buf.put_bytes(b'a', 4);

        assert_eq!(4, buf.remaining_mut());
        assert_eq!(b"aaaa\0\0", &dst[..]);
    }
}

#[test]
fn test_put_bytes_fills_remaining_capacity() {
    let mut dst = [core::mem::MaybeUninit::<u8>::uninit(); 6];
    unsafe {
        let buf = &mut UninitSlice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u8, dst.len());
        buf.put_bytes(b'a', 6);
        
        assert_eq!(0, buf.remaining_mut());
        assert_eq!(b"aaaaaa", &dst[..]);
    }
}


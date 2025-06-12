// Answer 0

#[test]
#[should_panic]
fn test_put_bytes_panic_remaining_mut_zero() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 4];
    unsafe {
        let uninit_slice = UninitSlice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len());
        uninit_slice.put_bytes(b'a', 1);
    }
}

#[test]
#[should_panic]
fn test_put_bytes_panic_remaining_mut_one() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 1];
    unsafe {
        let uninit_slice = UninitSlice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len());
        uninit_slice.put_bytes(b'a', 2);
    }
}

#[test]
fn test_put_bytes_no_panic() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 3];
    unsafe {
        let uninit_slice = UninitSlice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len());
        uninit_slice.put_bytes(b'a', 2);
    }
}

#[test]
#[should_panic]
fn test_put_bytes_exceed_capacity() {
    let mut buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 3];
    unsafe {
        let uninit_slice = UninitSlice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, 3);
        uninit_slice.put_bytes(b'a', 4);
    }
}


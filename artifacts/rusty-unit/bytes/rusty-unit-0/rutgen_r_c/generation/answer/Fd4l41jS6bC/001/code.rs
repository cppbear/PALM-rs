// Answer 0

#[test]
fn test_uninit_valid_slice() {
    let mut buffer = [MaybeUninit::uninit(); 64];
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer[..]);
    assert_eq!(slice.len(), 64);
}

#[test]
fn test_uninit_empty_slice() {
    let mut buffer: [MaybeUninit<u8>; 0] = [];
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer);
    assert_eq!(slice.len(), 0);
}

#[should_panic]
fn test_uninit_null_pointer() {
    unsafe {
        let slice: &mut UninitSlice = UninitSlice::uninit(&mut *(core::ptr::null_mut::<MaybeUninit<u8>>() as *mut [MaybeUninit<u8>]));
    }
}

#[test]
fn test_uninit_large_slice() {
    let mut buffer = [MaybeUninit::uninit(); 256];
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer[..]);
    assert_eq!(slice.len(), 256);
}


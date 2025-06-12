// Answer 0

#[test]
fn test_as_mut_ptr() {
    let mut data: [MaybeUninit<u8>; 3] = [
        MaybeUninit::new(0),
        MaybeUninit::new(1),
        MaybeUninit::new(2),
    ];
    let mut uninit_slice = UninitSlice::uninit(&mut data);
    let ptr = uninit_slice.as_mut_ptr();
    
    unsafe {
        // Ensure we can write to the memory at ptr
        *ptr = 10;

        // Ensure we can read the value we just wrote
        assert_eq!(*ptr, 10);
    }
}

#[test]
fn test_as_mut_ptr_length() {
    let mut data: [MaybeUninit<u8>; 5] = [
        MaybeUninit::new(1),
        MaybeUninit::new(2),
        MaybeUninit::new(3),
        MaybeUninit::new(4),
        MaybeUninit::new(5),
    ];
    let uninit_slice = UninitSlice::uninit(&mut data);
    
    assert_eq!(5, uninit_slice.len());
}

#[test]
#[should_panic]
fn test_as_mut_ptr_invalid_access() {
    let mut data: [MaybeUninit<u8>; 3] = [
        MaybeUninit::new(0),
        MaybeUninit::new(1),
        MaybeUninit::new(2),
    ];
    let mut uninit_slice = UninitSlice::uninit(&mut data);
    let ptr = uninit_slice.as_mut_ptr();
    
    unsafe {
        // This access is unsafe and should panic
        let invalid_access = &*(ptr as *const u8);
        let _ = *invalid_access; // Triggering read from uninitialized memory
    }
}


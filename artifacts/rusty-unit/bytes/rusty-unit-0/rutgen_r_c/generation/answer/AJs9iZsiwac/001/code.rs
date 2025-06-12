// Answer 0

#[test]
fn test_as_uninit_slice_mut() {
    let mut data: [MaybeUninit<u8>; 3] = [
        MaybeUninit::new(0),
        MaybeUninit::new(1),
        MaybeUninit::new(2),
    ];
    let mut uninit_slice = UninitSlice::uninit(&mut data);
    
    unsafe {
        let result: &mut [MaybeUninit<u8>] = uninit_slice.as_uninit_slice_mut();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].assume_init(), 0);
        assert_eq!(result[1].assume_init(), 1);
        assert_eq!(result[2].assume_init(), 2);
    }
}

#[test]
#[should_panic(expected = "uninitialized memory access")]
fn test_as_uninit_slice_mut_panic() {
    let mut data: [MaybeUninit<u8>; 3] = [
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
    ];
    let mut uninit_slice = UninitSlice::uninit(&mut data);
    
    unsafe {
        let result: &mut [MaybeUninit<u8>] = uninit_slice.as_uninit_slice_mut();
        // Accessing uninitialized memory should panic
        let _ = result[0].assume_init(); // This will trigger a panic
    }
}


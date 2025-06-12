// Answer 0

#[test]
fn test_len_non_empty_uninit_slice() {
    let mut data = [MaybeUninit::<u8>::uninit(); 5];
    let uninit_slice = UninitSlice::uninit(&mut data);
    assert_eq!(uninit_slice.len(), 5);
}

#[test]
fn test_len_empty_uninit_slice() {
    let mut data: [MaybeUninit<u8>; 0] = [];
    let uninit_slice = UninitSlice::uninit(&mut data);
    assert_eq!(uninit_slice.len(), 0);
}

#[test]
fn test_len_with_initialized_bytes() {
    let mut data = [0u8; 3];
    let uninit_slice = UninitSlice::new(&mut data);
    assert_eq!(uninit_slice.len(), 3);
}


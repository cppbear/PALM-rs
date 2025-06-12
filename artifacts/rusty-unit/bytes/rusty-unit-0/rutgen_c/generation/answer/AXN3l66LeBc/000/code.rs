// Answer 0

#[test]
fn test_len_zero_length() {
    let mut data: [MaybeUninit<u8>; 0] = [];
    let uninit_slice = UninitSlice::uninit(&mut data);
    assert_eq!(uninit_slice.len(), 0);
}

#[test]
fn test_len_non_zero_length() {
    let mut data: [MaybeUninit<u8>; 3] = [
        MaybeUninit::new(0),
        MaybeUninit::new(1),
        MaybeUninit::new(2),
    ];
    let uninit_slice = UninitSlice::uninit(&mut data);
    assert_eq!(uninit_slice.len(), 3);
}


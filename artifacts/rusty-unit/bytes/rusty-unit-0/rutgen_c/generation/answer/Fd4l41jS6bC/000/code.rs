// Answer 0

#[test]
fn test_uninit() {
    let mut buffer: [MaybeUninit<u8>; 64] = Default::default();
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer[..]);
    assert_eq!(slice.len(), 64);
}

#[test]
fn test_uninit_empty() {
    let mut buffer: [MaybeUninit<u8>; 0] = [];
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer[..]);
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_uninit_boundary() {
    let mut buffer: [MaybeUninit<u8>; 1] = [MaybeUninit::uninit()];
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer[..]);
    assert_eq!(slice.len(), 1);
}

#[test]
fn test_uninit_large_buffer() {
    let mut buffer: [MaybeUninit<u8>; 128] = Default::default();
    let slice: &mut UninitSlice = UninitSlice::uninit(&mut buffer[..]);
    assert_eq!(slice.len(), 128);
}


// Answer 0

#[test]
fn test_uninit_slice_new() {
    let mut buffer = [0u8; 64];
    let slice = UninitSlice::new(&mut buffer[..]);
    assert_eq!(slice.len(), 64);
}

#[test]
fn test_uninit_slice_new_empty() {
    let mut buffer: [u8; 0] = [];
    let slice = UninitSlice::new(&mut buffer[..]);
    assert_eq!(slice.len(), 0);
}

#[should_panic]
fn test_uninit_slice_new_invalid() {
    let buffer: &mut [u8] = &mut [];
    let _slice = UninitSlice::new(buffer); // Expecting a panic due to invalid buffer
}


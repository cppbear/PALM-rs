// Answer 0

#[test]
fn test_new_valid_slice() {
    let mut buffer = [0u8; 64];
    let slice = UninitSlice::new(&mut buffer[..]);
    assert_eq!(slice.len(), buffer.len());
}

#[test]
#[should_panic]
fn test_new_empty_slice() {
    let mut buffer: [u8; 0] = [];
    let _slice = UninitSlice::new(&mut buffer[..]);
}

#[test]
fn test_new_small_buffer() {
    let mut buffer = [0u8; 1];
    let slice = UninitSlice::new(&mut buffer[..]);
    assert_eq!(slice.len(), buffer.len());
}

#[test]
fn test_new_large_buffer() {
    let mut buffer = [0u8; 1024];
    let slice = UninitSlice::new(&mut buffer[..]);
    assert_eq!(slice.len(), buffer.len());
}


// Answer 0

#[test]
fn test_new_initializes_uninitslice() {
    let mut buffer = [0u8; 64];
    let slice = UninitSlice::new(&mut buffer[..]);
    
    assert_eq!(slice.len(), 64);
}

#[test]
fn test_new_with_empty_buffer() {
    let mut buffer: [u8; 0] = [];
    let slice = UninitSlice::new(&mut buffer[..]);
    
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_new_with_small_buffer() {
    let mut buffer = [1u8, 2u8, 3u8];
    let slice = UninitSlice::new(&mut buffer[..]);
    
    assert_eq!(slice.len(), 3);
}


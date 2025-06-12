// Answer 0

#[test]
fn test_as_mut_ptr_valid_slice() {
    let mut data = [0u8; 3];
    let mut slice = &mut data[..];
    let ptr = BufMut::chunk_mut(&mut slice).as_mut_ptr();
    assert_eq!(ptr, slice.as_mut_ptr());
}

#[test]
fn test_as_mut_ptr_empty_slice() {
    let mut data: [u8; 0] = [];
    let mut slice = &mut data[..];
    let ptr = BufMut::chunk_mut(&mut slice).as_mut_ptr();
    assert_eq!(ptr, slice.as_mut_ptr());
}

#[test]
#[should_panic]
fn test_as_mut_ptr_uninitialized_memory() {
    let mut slice: &mut [u8] = unsafe { std::mem::transmute::<&mut [u8; 1], &mut [u8; 1]>(std::ptr::null_mut()) };
    let _ptr = BufMut::chunk_mut(&mut slice).as_mut_ptr();
}

#[test]
fn test_as_mut_ptr_large_slice() {
    let mut data = [0u8; 1024];
    let mut slice = &mut data[..];
    let ptr = BufMut::chunk_mut(&mut slice).as_mut_ptr();
    assert_eq!(ptr, slice.as_mut_ptr());
}


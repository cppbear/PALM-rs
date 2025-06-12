// Answer 0

#[test]
fn test_copy_from_slice_equal_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.copy_from_slice(b"abc");
    
    assert_eq!(b"abc", &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_different_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.copy_from_slice(b"abcd");
}

#[test]
fn test_copy_from_slice_zero_length() {
    let mut data: [u8; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 0) };
    
    slice.copy_from_slice(&[]);

    assert_eq!(&[], &data[..]);
}


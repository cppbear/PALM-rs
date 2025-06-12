// Answer 0

#[test]
fn test_copy_from_slice_success() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.copy_from_slice(b"abc");
    
    assert_eq!(b"abc", &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_panics_on_different_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.copy_from_slice(b"abcd"); // Length of source is different from destination
}

#[test]
fn test_copy_from_slice_empty_success() {
    let mut data: [u8; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 0) };
    
    slice.copy_from_slice(&[]); // Both source and destination are empty
    
    assert_eq!(&[], &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_panics_on_empty_source() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.copy_from_slice(&[]); // Source is empty but destination length is 3
}

#[test]
fn test_copy_from_slice_boundary_conditions() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };
    
    slice.copy_from_slice(b"12345");
    
    assert_eq!(b"12345", &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_boundary_panics() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };
    
    slice.copy_from_slice(b"123"); // Source length is less than destination length
}


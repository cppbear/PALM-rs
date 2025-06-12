// Answer 0

#[test]
fn test_copy_from_slice_success() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.copy_from_slice(b"foo");

    assert_eq!(b"foo", &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_length_mismatch() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.copy_from_slice(b"foobar");
}

#[test]
fn test_copy_from_slice_empty() {
    let mut data: [u8; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 0) };

    slice.copy_from_slice(b"");

    assert_eq!(&data[..], &[]);
}

#[test]
fn test_copy_from_slice_single_byte() {
    let mut data = [0u8; 1];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 1) };

    slice.copy_from_slice(b"x");

    assert_eq!(b"x", &data[..]);
}


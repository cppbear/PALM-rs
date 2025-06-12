// Answer 0

#[test]
fn test_copy_from_slice_panics_len_mismatch_smaller_src() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), data.len()) };
    slice.copy_from_slice(&[1, 2]); // src.len() is 2, self.len() is 5, should panic
}

#[test]
fn test_copy_from_slice_panics_len_mismatch_larger_src() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), data.len()) };
    slice.copy_from_slice(&[1, 2, 3, 4, 5, 6]); // src.len() is 6, self.len() is 5, should panic
}

#[test]
fn test_copy_from_slice_exact_length() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), data.len()) };
    slice.copy_from_slice(&[1, 2, 3, 4, 5]); // src.len() is 5, self.len() is 5
}

#[test]
fn test_copy_from_slice_zero_length() {
    let mut data: [u8; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), data.len()) };
    slice.copy_from_slice(&[]); // src.len() is 0, self.len() is 0
}

#[test]
fn test_copy_from_slice_non_zero_length() {
    let mut data = [0u8; 10];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), data.len()) };
    slice.copy_from_slice(b"hello\x00\x00\x00\x00"); // src.len() is 10, self.len() is 10, with null bytes
}


// Answer 0

#[test]
fn test_write_byte_panic_index_equal_to_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(3, 1); // This should panic as index equals the length of the slice
}

#[test]
fn test_write_byte_panic_index_greater_than_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(4, 1); // This should panic as index is greater than the length of the slice
}


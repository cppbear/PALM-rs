// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(0, b'a');
    assert_eq!(data[0], b'a'); // Test first element

    slice.write_byte(1, b'b');
    assert_eq!(data[1], b'b'); // Test second element

    slice.write_byte(2, b'c');
    assert_eq!(data[2], b'c'); // Test third element
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_write_byte_out_of_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(3, b'd'); // This should panic
}

#[test]
fn test_write_byte_to_zero_length_slice() {
    let mut data: [MaybeUninit<u8>; 0] = [];
    let slice = UninitSlice::uninit(&mut data);

    assert_eq!(slice.len(), 0);
    
    // The len is 0, writing should panic
    // It will not panic on assertion caused by the index check
    #[should_panic(expected = "assertion failed")]
    {
        slice.write_byte(0, b'e'); // This should panic
    }
}


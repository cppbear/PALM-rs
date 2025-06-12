// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };

    slice.write_byte(0, b'a');
    assert_eq!(data[0], b'a');
    
    slice.write_byte(1, b'b');
    assert_eq!(data[1], b'b');

    slice.write_byte(2, b'c');
    assert_eq!(data[2], b'c');

    slice.write_byte(3, b'd');
    assert_eq!(data[3], b'd');

    slice.write_byte(4, b'e');
    assert_eq!(data[4], b'e');
}

#[should_panic]
#[test]
fn test_write_byte_out_of_bounds() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };

    slice.write_byte(5, b'f'); // This should panic as index == len
}


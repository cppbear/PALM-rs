// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(0, b'a');
    slice.write_byte(1, b'b');
    slice.write_byte(2, b'c');

    assert_eq!(&data[..], &[b'a', b'b', b'c']);
}

#[should_panic]
#[test]
fn test_write_byte_out_of_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(3, b'd'); // This should panic as the index is out of bounds
}

#[test]
fn test_write_byte_edge_case() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(2, b'e');

    assert_eq!(&data[..], &[0, 0, b'e']);
}

#[test]
fn test_write_byte_multiple_calls() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };

    slice.write_byte(0, b'x');
    slice.write_byte(1, b'y');
    slice.write_byte(2, b'z');

    assert_eq!(&data[..], &[b'x', b'y', b'z', 0, 0]);
}


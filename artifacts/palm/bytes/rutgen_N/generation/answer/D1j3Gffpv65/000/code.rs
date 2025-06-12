// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(0, b'b');

    assert_eq!(&data[..], b"boo");
}

#[test]
#[should_panic]
fn test_write_byte_out_of_bounds() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(3, b'b'); // This should panic as index 3 is out of bounds.
}

#[test]
fn test_write_byte_at_last_index() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(2, b'r');

    assert_eq!(&data[..], b"foor");
}


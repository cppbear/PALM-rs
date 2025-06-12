// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(0, b'b');
    assert_eq!(b"boo", &data[..]);

    slice.write_byte(1, b'a');
    assert_eq!(b"bao", &data[..]);

    slice.write_byte(2, b'z');
    assert_eq!(b"baz", &data[..]);
}

#[should_panic]
#[test]
fn test_write_byte_out_of_bounds() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    // This should panic because index 3 is out of bounds for length 3
    slice.write_byte(3, b'x');
}


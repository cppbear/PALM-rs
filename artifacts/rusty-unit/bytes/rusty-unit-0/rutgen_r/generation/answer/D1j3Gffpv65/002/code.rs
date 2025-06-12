// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(0, b'b');

    assert_eq!(b"boo", &data[..]);
}

#[test]
#[should_panic]
fn test_write_byte_out_of_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    // This should panic since index == self.len() (3)
    slice.write_byte(3, b'b');
}


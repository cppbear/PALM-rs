// Answer 0

#[test]
fn test_write_byte_within_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(0, 1);
    slice.write_byte(1, 2);
    slice.write_byte(2, 3);
}

#[test]
#[should_panic]
fn test_write_byte_out_of_bounds() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.write_byte(3, 1);
}

#[test]
fn test_write_byte_boundary_value() {
    let mut data = [0u8; 1];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 1) };

    slice.write_byte(0, 255);
}

#[test]
fn test_write_byte_multiple_operations() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };

    slice.write_byte(0, 100);
    slice.write_byte(1, 101);
    slice.write_byte(2, 102);
    slice.write_byte(3, 103);
    slice.write_byte(4, 104);
}


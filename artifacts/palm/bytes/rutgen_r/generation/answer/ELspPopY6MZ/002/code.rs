// Answer 0

#[test]
fn test_copy_from_slice_success() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.copy_from_slice(b"bar");

    assert_eq!(b"bar", &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_panic_length_mismatch() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.copy_from_slice(b"barbaz"); // src is longer than self
}

#[test]
#[should_panic]
fn test_copy_from_slice_panic_empty_length() {
    let mut empty_data: [u8; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(empty_data.as_mut_ptr(), 0) };

    slice.copy_from_slice(b""); // Both src and self are empty, should not panic
}

#[test]
#[should_panic]
fn test_copy_from_slice_panic_src_empty() {
    let mut data = [b'f', b'o', b'o'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };

    slice.copy_from_slice(b""); // src is empty, self is not
}

#[test]
fn test_copy_from_slice_edge_case_single_byte() {
    let mut data = [b'x'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 1) };

    slice.copy_from_slice(b'y');

    assert_eq!(b'y', &data[..]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_panic_src_different_length() {
    let mut data = [b'a', b'b'];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 2) };

    slice.copy_from_slice(b"abc"); // src is longer than self
}


// Answer 0

#[test]
fn test_as_slice_mut_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0);
    let slice = bytes_mut.as_slice_mut();
    assert_eq!(slice.len(), 5);
    assert_eq!(slice[0], 0);
}

#[test]
fn test_as_slice_mut_empty() {
    let mut bytes_mut = BytesMut::new();
    let slice = bytes_mut.as_slice_mut();
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_as_slice_mut_after_resize() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(3, 1);
    let slice = bytes_mut.as_slice_mut();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], 1);
}

#[test]
#[should_panic]
fn test_as_slice_mut_invalid_access() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        let slice = bytes_mut.as_slice_mut();
        // Trying to access an index that doesn't exist should panic
        let _value = slice[0]; // This line should panic since the slice is empty.
    }
}


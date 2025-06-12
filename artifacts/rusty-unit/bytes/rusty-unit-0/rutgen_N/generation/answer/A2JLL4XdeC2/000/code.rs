// Answer 0

#[test]
fn test_as_mut_ptr_uninitialized_memory() {
    struct TestBufMut<'a>(&'a mut [u8]);

    let mut data = [0u8, 1, 2];
    let mut slice = &mut data[..];

    let ptr = TestBufMut(slice).0.as_mut_ptr();

    // Ensure the pointer is not null and points to the first element of the slice.
    assert!(!ptr.is_null());
    assert_eq!(unsafe { *ptr }, 0);
}

#[test]
fn test_as_mut_ptr_empty_slice() {
    struct TestBufMut<'a>(&'a mut [u8]);

    let mut data: &[u8] = &mut [];
    let ptr = TestBufMut(data).0.as_mut_ptr();

    // Ensure the pointer is null for an empty slice.
    assert!(ptr.is_null());
}


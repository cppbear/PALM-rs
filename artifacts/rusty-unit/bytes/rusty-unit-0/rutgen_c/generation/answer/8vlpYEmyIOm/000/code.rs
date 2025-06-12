// Answer 0

#[test]
fn test_new_empty_with_ptr_non_null() {
    let ptr = Box::into_raw(Box::new([0u8; 0])) as *const u8; // Create a non-null pointer.
    let bytes = Bytes::new_empty_with_ptr(ptr);
    assert_eq!(bytes.len, 0);
    assert!(!bytes.ptr.is_null());
}

#[test]
#[should_panic]
fn test_new_empty_with_ptr_null() {
    let ptr: *const u8 = std::ptr::null(); // Create a null pointer.
    let _ = Bytes::new_empty_with_ptr(ptr); // This should panic due to debug_assert.
}

#[test]
fn test_new_empty_with_ptr_validity() {
    let ptr = Box::into_raw(Box::new([0u8; 0])) as *const u8; // Create a non-null pointer.
    let bytes = Bytes::new_empty_with_ptr(ptr);
    assert_eq!(bytes.ptr as usize, without_provenance(ptr as usize) as usize);
    assert_eq!(bytes.len, 0);
}


// Answer 0

#[test]
#[should_panic]
fn test_new_empty_with_null_ptr() {
    let ptr: *const u8 = core::ptr::null();
    let _result = Bytes::new_empty_with_ptr(ptr);
}

#[test]
fn test_new_empty_with_non_null_ptr() {
    let ptr: *const u8 = &0u8 as *const u8; // Valid non-null pointer
    let result = Bytes::new_empty_with_ptr(ptr);
    assert_eq!(result.len, 0);
    assert_eq!(result.ptr as usize, without_provenance(ptr as usize) as usize);
}

#[test]
#[should_panic]
fn test_new_empty_with_zero_size_blank_ptr() {
    let zero_size_ptr: *const u8 = &0u8 as *const u8; // Non-null but treated as ZST in context
    let _result = Bytes::new_empty_with_ptr(zero_size_ptr);
}


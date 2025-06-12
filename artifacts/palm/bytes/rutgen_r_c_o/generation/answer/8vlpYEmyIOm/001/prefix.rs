// Answer 0

#[should_panic]
fn test_new_empty_with_null_ptr() {
    let ptr: *const u8 = std::ptr::null();
    let bytes = Bytes::new_empty_with_ptr(ptr);
}

#[should_panic]
fn test_new_empty_with_pointer_bounds() {
    let ptr: *const u8 = std::ptr::null();
    let bytes = Bytes::new_empty_with_ptr(ptr);
}


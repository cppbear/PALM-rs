// Answer 0

#[test]
fn test_vptr_non_null_ptr() {
    let value: u8 = 42;
    let ptr: *mut u8 = &mut value as *mut u8;
    let non_null_ptr = vptr(ptr);
    assert_eq!(unsafe { *non_null_ptr.as_ptr() }, 42);
}

#[test]
#[should_panic(expected = "Vec pointer should be non-null")]
fn test_vptr_null_ptr() {
    let ptr: *mut u8 = std::ptr::null_mut();
    vptr(ptr);
}

#[test]
fn test_vptr_invalid_non_null_ptr() {
    let value: u8 = 0;
    let invalid_ptr: *mut u8 = &value as *const u8 as *mut u8;
    let non_null_ptr = vptr(invalid_ptr);
    assert_eq!(unsafe { *non_null_ptr.as_ptr() }, 0);
}


// Answer 0

#[test]
fn test_vptr_non_null() {
    let mut value = 42u8;
    let ptr = &mut value as *mut u8;
    let non_null_ptr = vptr(ptr);
    assert_eq!(non_null_ptr.as_ptr(), ptr);
}

#[test]
#[should_panic(expected = "Vec pointer should be non-null")]
fn test_vptr_null_pointer() {
    let null_ptr: *mut u8 = core::ptr::null_mut();
    let _non_null_ptr = vptr(null_ptr);
}


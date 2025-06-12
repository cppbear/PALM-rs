// Answer 0

#[test]
fn test_vptr_non_null() {
    let mut value: u8 = 42;
    let ptr: *mut u8 = &mut value;

    let non_null_ptr = vptr(ptr);
    assert_eq!(unsafe { *non_null_ptr.as_ptr() }, 42);
}

#[test]
#[should_panic(expected = "Vec pointer should be non-null")]
fn test_vptr_null_pointer() {
    let ptr: *mut u8 = std::ptr::null_mut();
    
    vptr(ptr); // This should panic in debug mode
}


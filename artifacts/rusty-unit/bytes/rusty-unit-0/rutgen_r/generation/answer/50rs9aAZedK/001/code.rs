// Answer 0

#[test]
fn test_owned_to_vec() {
    use std::ffi::CString;
    use std::mem;
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};

    // Create test data
    let test_str = CString::new("Hello, World!").unwrap();
    let ptr = test_str.as_ptr();
    let len = test_str.to_bytes().len();

    // Create an AtomicPtr pointing to a test value
    let data = AtomicPtr::new(Box::into_raw(Box::new(0u8)));

    // Ensure the returned Vec matches the expected bytes
    let vec = unsafe { owned_to_vec(&data, ptr, len) };
    
    assert_eq!(vec, vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]);

    // Clean up: Assuming owned_drop_impl requires proper memory handling
    unsafe {
        let _ = Box::from_raw(data.load(Ordering::Relaxed));
    }
}

#[test]
#[should_panic]
fn test_owned_to_vec_with_zero_length() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(Box::into_raw(Box::new(0u8)));
    let ptr = null();
    let len = 0;

    // Expect a panic when attempting to create a Vec from a zero-length slice
    unsafe {
        owned_to_vec(&data, ptr, len);
    }
}


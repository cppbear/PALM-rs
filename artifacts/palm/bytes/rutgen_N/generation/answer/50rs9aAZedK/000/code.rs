// Answer 0

#[test]
fn test_owned_to_vec() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::slice;
    use std::ptr;

    // Initialize a slice to convert to Vec
    let data_slice: &[u8] = &[1, 2, 3, 4, 5];
    let len = data_slice.len();
    
    // Create a raw pointer to the slice's data
    let ptr = data_slice.as_ptr();

    // Use an atomic pointer for owned data
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());

    // Simulate ownership handling
    unsafe fn owned_drop_impl(_: *mut ()) {}

    // Call the function to be tested
    let result_vec = unsafe { owned_to_vec(&atomic_ptr, ptr, len) };

    // Check that the resulting vector has the expected contents
    assert_eq!(result_vec, vec![1, 2, 3, 4, 5]);
}

#[test]
#[should_panic]
fn test_owned_to_vec_with_zero_length() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::slice;
    use std::ptr;

    // Create a null pointer and length of zero
    let ptr = ptr::null();
    let len = 0;

    // Use an atomic pointer for owned data
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());

    // Simulate ownership handling
    unsafe fn owned_drop_impl(_: *mut ()) {}

    // Call the function to be tested, expecting it to panic
    let _result_vec = unsafe { owned_to_vec(&atomic_ptr, ptr, len) };
}


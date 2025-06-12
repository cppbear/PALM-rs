// Answer 0

#[test]
fn test_owned_to_vec_valid_pointer() {
    use std::ptr;
    use std::sync::atomic::AtomicPtr;

    let data: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());

    // Create a temporary buffer and obtain a pointer to it
    let temp_buf = vec![1u8, 2, 3, 4];
    let ptr = temp_buf.as_ptr();

    // Call the function under test
    let result = unsafe { owned_to_vec(&data, ptr, temp_buf.len()) };

    // Verify the result
    assert_eq!(result, temp_buf);
}

#[test]
#[should_panic]
fn test_owned_to_vec_zero_length() {
    use std::ptr;
    use std::sync::atomic::AtomicPtr;

    let data: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());

    // Here we pass a null pointer for zero length
    let result = unsafe { owned_to_vec(&data, ptr::null(), 0) };

    // Verify that an empty vector is returned
    assert_eq!(result.len(), 0);
}

#[test]
fn test_owned_to_vec_non_empty_data() {
    use std::ptr;
    use std::sync::atomic::AtomicPtr;

    let data: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());

    // Using a temporary buffer with some sample data
    let temp_buf = vec![5u8, 6, 7, 8, 9];
    let ptr = temp_buf.as_ptr();

    // Call the function under test
    let result = unsafe { owned_to_vec(&data, ptr, temp_buf.len()) };

    // Verify the result
    assert_eq!(result, temp_buf);
}


// Answer 0

#[test]
fn test_static_to_vec_with_valid_pointer() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let len = data.len();
    let ptr = data.as_ptr();

    // We need to ensure that the data is valid while calling static_to_vec, so we drop the Vec after getting the pointer.
    let _data_box = ManuallyDrop::new(data);

    unsafe {
        let result = static_to_vec(&AtomicPtr::new(ptr as *mut ()), ptr, len);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}

#[test]
#[should_panic]
fn test_static_to_vec_with_zero_length() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();

    let _data_box = ManuallyDrop::new(data);

    unsafe {
        let result = static_to_vec(&AtomicPtr::new(ptr as *mut ()), ptr, 0);
        assert!(result.is_empty()); // This will not panic.
    }
}

#[test]
#[should_panic]
fn test_static_to_vec_with_invalid_pointer() {
    let invalid_ptr: *const u8 = std::ptr::null();

    unsafe {
        let _result = static_to_vec(&AtomicPtr::new(invalid_ptr as *mut ()), invalid_ptr, 5); // This should cause a panic.
    }
}


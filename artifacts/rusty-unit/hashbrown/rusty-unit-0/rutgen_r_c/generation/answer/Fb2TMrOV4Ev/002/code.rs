// Answer 0

#[test]
fn test_next_n_non_zero_sized() {
    use core::ptr::NonNull;

    struct TestType;

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let base_ptr: *mut TestType = &mut TestType as *mut TestType;

    // Create a NonNull pointer from the base pointer
    let non_null_base = NonNull::new(base_ptr).unwrap();

    let bucket = Bucket { ptr: non_null_base };

    // Test with a normal offset
    let offset: usize = 1;
    let new_bucket = unsafe { bucket.next_n(offset) };
    assert!(new_bucket.ptr.as_ptr() == (base_ptr as usize - offset * core::mem::size_of::<TestType>()) as *mut TestType);
}

#[test]
#[should_panic]
fn test_next_n_panic_invalid_pointer() {
    use core::ptr::NonNull;

    struct TestType;

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let base_ptr: *mut TestType = &mut TestType as *mut TestType;

    // Create a NonNull pointer from the base pointer
    let non_null_base = NonNull::new(base_ptr).unwrap();

    let bucket = Bucket { ptr: non_null_base };

    // In this case we provide an invalid offset that exceeds the bounds
    let invalid_offset: usize = usize::MAX; // Intentionally invalid offset
    unsafe {
        // This is expected to panic due to the invalid offset calculation
        let _ = bucket.next_n(invalid_offset);
    }
}

#[test]
fn test_next_n_zero_offset() {
    use core::ptr::NonNull;

    struct TestType;

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    let base_ptr: *mut TestType = &mut TestType as *mut TestType;

    // Create a NonNull pointer from the base pointer
    let non_null_base = NonNull::new(base_ptr).unwrap();

    let bucket = Bucket { ptr: non_null_base };

    // Test with a zero offset, should return the same bucket
    let offset: usize = 0;
    let new_bucket = unsafe { bucket.next_n(offset) };
    assert!(new_bucket.ptr.as_ptr() == base_ptr);
}

#[test]
fn test_next_n_large_offset() {
    use core::ptr::NonNull;

    struct TestType;

    impl TestType {
        const IS_ZERO_SIZED: bool = false;
    }

    // Creating enough space for testing large offsets
    let mut test_storage: Vec<TestType> = Vec::with_capacity(10);
    let base_ptr: *mut TestType = test_storage.as_mut_ptr();

    // Create a NonNull pointer from the base pointer
    let non_null_base = NonNull::new(base_ptr).unwrap();

    let bucket = Bucket { ptr: non_null_base };

    // Test with an appropriate large offset
    let offset: usize = 5; // Ensuring it's within bounds
    let new_bucket = unsafe { bucket.next_n(offset) };
    assert!(new_bucket.ptr.as_ptr() == (base_ptr as usize - offset * core::mem::size_of::<TestType>()) as *mut TestType);
}


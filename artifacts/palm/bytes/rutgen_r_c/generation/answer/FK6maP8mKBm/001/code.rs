// Answer 0

#[test]
fn test_shared_to_vec_impl_unique_ref_count() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe {
        // Create a buffer with a size of 10 bytes
        let buffer_size = 10;
        let buf = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(buffer_size, 1).unwrap());
        let input_data = b"abcdefghij"; // 10 bytes

        // Initialize the buffer with data
        ptr::copy_nonoverlapping(input_data.as_ptr(), buf, buffer_size);

        // Set up the Shared structure with unique ref_count
        let shared = Box::into_raw(Box::new(TestShared {
            buf,
            cap: buffer_size,
            ref_cnt: AtomicUsize::new(1), // Unique reference
        }));

        // Call the unsafe function
        let result_vec = shared_to_vec_impl(shared, buf, buffer_size);

        // Verify the result
        assert_eq!(result_vec.len(), buffer_size);
        assert_eq!(&result_vec[..], input_data);

        // Clean up the allocated buffer
        alloc::alloc::dealloc(buf, alloc::alloc::Layout::from_size_align(buffer_size, 1).unwrap());
    }
}

#[test]
fn test_shared_to_vec_impl_non_unique_ref_count() {
    struct TestShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    unsafe {
        // Create a buffer with a size of 10 bytes
        let buffer_size = 10;
        let buf = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(buffer_size, 1).unwrap());
        let input_data = b"abcdefghij"; // 10 bytes

        // Initialize the buffer with data
        ptr::copy_nonoverlapping(input_data.as_ptr(), buf, buffer_size);

        // Set up the Shared structure with non-unique ref_count
        let shared = Box::into_raw(Box::new(TestShared {
            buf,
            cap: buffer_size,
            ref_cnt: AtomicUsize::new(2), // Non-unique reference
        }));

        // Call the unsafe function
        let result_vec = shared_to_vec_impl(shared, buf, buffer_size);

        // Verify the result
        assert_eq!(result_vec.len(), buffer_size);
        assert_eq!(&result_vec[..], input_data);

        // Clean up the allocated buffer
        alloc::alloc::dealloc(buf, alloc::alloc::Layout::from_size_align(buffer_size, 1).unwrap());
    }
}

#[should_panic]
#[test]
fn test_shared_to_vec_impl_invalid_shared_pointer() {
    unsafe {
        let invalid_shared: *mut Shared = std::ptr::null_mut();
        let result_vec = shared_to_vec_impl(invalid_shared, std::ptr::null(), 0);
        assert!(result_vec.is_empty());
    }
}


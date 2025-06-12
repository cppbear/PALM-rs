// Answer 0

#[test]
fn test_shared_to_vec_impl_non_unique_ref_count() {
    use std::ptr;

    // Define the Shared struct and create an instance with a reference count greater than 1
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Create a buffer
    let cap = 10;
    let buf: *mut u8 = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(cap, 1).unwrap()) };
    
    // Initialize the shared structure with ref_cnt > 1
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap,
        ref_cnt: AtomicUsize::new(2),
    }));

    // Create a pointer to data that will be copied
    let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ptr = data.as_ptr();
    let len = data.len();

    // Invoke the function under test
    let result = unsafe { shared_to_vec_impl(shared, ptr, len) };

    // Verify the result is as expected (a new Vec containing the original data)
    assert_eq!(result.as_slice(), data);

    // Ensure the original buffer was not modified
    unsafe {
        let original_slice = std::slice::from_raw_parts(buf, cap);
        assert_ne!(original_slice, data);
    }

    // Cleanup the allocated buffer
    unsafe {
        std::alloc::dealloc(buf, std::alloc::Layout::from_size_align(cap, 1).unwrap());
    }
}

#[test]
#[should_panic]
fn test_shared_to_vec_impl_invalid_shared() {
    use std::ptr;

    // Define a Shared struct with an invalid pointer (for panic simulation)
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Create a pointer to invalid shared data
    let shared: *mut Shared = ptr::null_mut();
    let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ptr = data.as_ptr();
    let len = data.len();

    // Invoke the function under test; it should panic
    let _ = unsafe { shared_to_vec_impl(shared, ptr, len) };
}


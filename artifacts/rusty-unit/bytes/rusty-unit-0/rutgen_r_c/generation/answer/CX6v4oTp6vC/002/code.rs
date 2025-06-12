// Answer 0

#[test]
fn test_promotable_odd_clone_with_kind_vec() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicPtr;

    // Prepare a shared buffer for the test.
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example data
    let buffer_ptr = buffer.as_ptr() as *mut u8; // Cast to *mut u8 for shared buffer

    // Create an AtomicPtr with KIND_VEC (0b1)
    let shared = Box::new(Shared {
        buf: buffer_ptr,
        cap: buffer.len(),
        ref_cnt: AtomicUsize::new(1),
    });
    let shared_ptr = Box::into_raw(shared); // Convert Box to raw pointer

    // Create the AtomicPtr with kind being KIND_VEC
    let data = AtomicPtr::new(shared_ptr as _);

    // Define pointer and length for cloning
    let clone_ptr = buffer_ptr;
    let clone_len = buffer.len();

    // Call the function under test
    let cloned_bytes = unsafe { promotable_odd_clone(&data, clone_ptr, clone_len) };

    // Validate results
    assert_eq!(cloned_bytes.len, clone_len);
    assert_eq!(cloned_bytes.ptr, clone_ptr);

    let shared_clone = cloned_bytes.data.load(Ordering::Acquire) as *mut Shared;
    assert!(!shared_clone.is_null());

    // Verify reference count increased
    assert_eq!((*shared_clone).ref_cnt.load(Ordering::Relaxed), 2);
}

#[test]
#[should_panic]
fn test_promotable_odd_clone_panic_due_to_invalid_kind() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicPtr;

    // Create an AtomicPtr with an invalid kind (0b0)
    let shared = Box::new(Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared_ptr = Box::into_raw(shared); // Convert Box to raw pointer

    // Create the AtomicPtr with kind being KIND_ARC (0b0)
    let data = AtomicPtr::new(shared_ptr as _);

    // Define pointer and length for cloning
    let clone_ptr = null_mut();
    let clone_len = 0;

    // Call the function under test, should panic
    let _ = unsafe { promotable_odd_clone(&data, clone_ptr, clone_len) };
}


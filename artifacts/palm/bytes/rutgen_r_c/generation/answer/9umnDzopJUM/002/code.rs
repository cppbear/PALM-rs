// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    use core::ptr::null_mut;

    // Prepare test data
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5];
    let atom = AtomicPtr::new(null_mut());
    let ptr = NonNull::from(&buffer).as_ptr() as *const ();
    let buf = buffer.as_ptr() as *mut u8;
    let offset = buf.add(2); // Offset pointing to the third element
    let len = 3; // Length for the remaining elements

    // Attempt to clone
    let result = unsafe { shallow_clone_vec(&atom, ptr, buf, offset, len) };

    // Assertions for expected results
    assert_eq!(result.len, len);
    assert_eq!(result.ptr as *const u8, offset);
    assert!(result.data.load(Ordering::Relaxed).is_null() == false);
    assert_eq!(result.vtable, &SHARED_VTABLE);
}

#[test]
#[should_panic]
fn test_shallow_clone_vec_alignment_failure() {
    use core::ptr::null_mut;

    // Prepare test data that violates alignment
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5];
    let atom = AtomicPtr::new(null_mut());
    let ptr = NonNull::from(&buffer).as_ptr() as *const ();
    let buf = buffer.as_ptr() as *mut u8;
    let offset = buf.add(1); // Offset pointing to a misaligned position
    let len = 3; // Length for the remaining elements

    // This should panic due to alignment assertion
    unsafe { shallow_clone_vec(&atom, ptr, buf, offset, len) };
}


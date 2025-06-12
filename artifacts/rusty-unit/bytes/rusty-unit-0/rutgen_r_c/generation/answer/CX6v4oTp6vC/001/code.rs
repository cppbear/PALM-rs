// Answer 0

#[test]
fn test_promotable_odd_clone_kind_arc() {
    use core::ptr::null_mut;

    // Create a shared memory block
    let buf = Box::into_raw(Box::new([0u8; 10])); // Buffer of size 10
    let shared = Box::new(Shared {
        buf,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    });

    // Point to the shared memory
    let atom = AtomicPtr::new(shared as *mut _);
    let ptr = buf as *const u8; // Pointer to the buffer
    let len = 10; // Length of the buffer

    // Perform the promotable_odd_clone operation
    let clone_result = unsafe { promotable_odd_clone(&atom, ptr, len) };

    // Check the result
    assert_eq!(clone_result.len, len);
    assert_eq!(clone_result.ptr, ptr);
    assert_eq!(unsafe { (*clone_result.data.load(Ordering::Acquire)).ref_cnt.load(Ordering::Relaxed) }, 2);

    // Clean up
    unsafe {
        let _ = Box::from_raw(clone_result.data.load(Ordering::Acquire));
        let _ = Box::from_raw(shared);
        let _ = Box::from_raw(buf);
    }
}


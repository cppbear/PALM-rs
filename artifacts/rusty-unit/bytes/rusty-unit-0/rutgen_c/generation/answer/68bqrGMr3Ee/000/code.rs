// Answer 0

#[test]
fn test_shallow_clone_arc() {
    use core::ptr::null_mut;

    struct LocalShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Initialize a Shared structure with dummy values
    let initial_buf: *mut u8 = null_mut();
    let initial_cap: usize = 10;
    let initial_ref_cnt = AtomicUsize::new(1);

    // Create an instance of LocalShared
    let shared = LocalShared {
        buf: initial_buf,
        cap: initial_cap,
        ref_cnt: initial_ref_cnt,
    };

    // Create a mutable pointer to LocalShared
    let shared_ptr: *mut LocalShared = &mut shared as *mut _;

    // Call the function under test
    let result = unsafe { shallow_clone_arc(shared_ptr as *mut Shared, initial_buf, initial_cap) };

    // Assertions
    assert_eq!(result.ptr, initial_buf);
    assert_eq!(result.len, initial_cap);
    assert!(result.data.load(Ordering::Relaxed) == shared_ptr as *mut ());

    // Clean up
    let new_ref_count = unsafe { (*shared_ptr).ref_cnt.load(Ordering::Relaxed) };
    assert_eq!(new_ref_count, 2); // The ref_cnt should have incremented
}

#[test]
#[should_panic]
fn test_shallow_clone_arc_panic_on_overflow() {
    use core::ptr::null_mut;

    struct LocalShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Initialize a Shared structure with ref_cnt close to the overflow limit
    let initial_buf: *mut u8 = null_mut();
    let initial_cap: usize = 10;
    let initial_ref_cnt = AtomicUsize::new(usize::MAX >> 1); // Set to maximum allowable before panic

    // Create an instance of LocalShared
    let shared = LocalShared {
        buf: initial_buf,
        cap: initial_cap,
        ref_cnt: initial_ref_cnt,
    };

    // Create a mutable pointer to LocalShared
    let shared_ptr: *mut LocalShared = &mut shared as *mut _;

    // This should panic due to overflow
    unsafe { shallow_clone_arc(shared_ptr as *mut Shared, initial_buf, initial_cap) };
}


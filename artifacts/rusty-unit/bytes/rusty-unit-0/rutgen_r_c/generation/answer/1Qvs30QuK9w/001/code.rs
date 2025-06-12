// Answer 0

#[test]
fn test_owned_drop_valid_ref_count() {
    use core::sync::atomic::AtomicUsize;

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    // Mock drop function to avoid actual deallocation
    unsafe extern "C" fn mock_drop(_ptr: *mut ()) {}

    let lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(1),
        drop: mock_drop,
    };

    let owned_ptr = &lifetime as *const OwnedLifetime as *mut ();
    let atomic_ptr = AtomicPtr::new(owned_ptr);
    
    // Call owned_drop safely with a valid pointer and reference count.
    unsafe {
        owned_drop(&mut atomic_ptr, core::ptr::null(), 0);
    }
}

#[test]
#[should_panic]
fn test_owned_drop_ref_count_underflow() {
    use core::sync::atomic::AtomicUsize;

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    // Mock drop function to avoid actual deallocation
    unsafe extern "C" fn mock_drop(_ptr: *mut ()) {}

    let lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(0), // Set initial reference count to 0 to trigger panic
        drop: mock_drop,
    };

    let owned_ptr = &lifetime as *const OwnedLifetime as *mut ();
    let atomic_ptr = AtomicPtr::new(owned_ptr);

    // Attempting to drop should panic due to invalid ref count
    unsafe {
        owned_drop(&mut atomic_ptr, core::ptr::null(), 0);
    }
}

#[test]
fn test_owned_drop_multiple_references() {
    use core::sync::atomic::AtomicUsize;

    struct OwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: fn(*mut ()),
    }

    // Mock drop function to avoid actual deallocation
    unsafe extern "C" fn mock_drop(_ptr: *mut ()) {}

    let lifetime = OwnedLifetime {
        ref_cnt: AtomicUsize::new(2), // Set reference count to 2 to simulate another hold
        drop: mock_drop,
    };

    let owned_ptr = &lifetime as *const OwnedLifetime as *mut ();
    let atomic_ptr = AtomicPtr::new(owned_ptr);

    // Call to owned_drop should succeed and decrease ref count
    unsafe {
        owned_drop(&mut atomic_ptr, core::ptr::null(), 0);
    }
    
    // Verify that the ref count is decremented correctly
    assert_eq!(lifetime.ref_cnt.load(core::sync::atomic::Ordering::Acquire), 1);
}

#[test]
#[should_panic]
fn test_owned_drop_invalid_pointer() {
    let invalid_ptr: *mut () = core::ptr::null_mut();
    let atomic_ptr = AtomicPtr::new(invalid_ptr);

    // This should panic when trying to drop an invalid reference
    unsafe {
        owned_drop(&mut atomic_ptr, core::ptr::null(), 0);
    }
}


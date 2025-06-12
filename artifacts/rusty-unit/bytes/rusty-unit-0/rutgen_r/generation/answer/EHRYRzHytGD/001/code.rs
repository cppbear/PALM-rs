// Answer 0

#[test]
fn test_release_shared_not_last_reference() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    let shared = Box::new(Shared {
        ref_count: AtomicUsize::new(2), // Initially 2 references
    });

    let ptr = Box::into_raw(shared);
    
    // Call the unsafe function
    unsafe {
        release_shared(ptr);
    }

    // Verify the reference count is now 1
    assert_eq!((*ptr).ref_count.load(Ordering::Relaxed), 1);

    // Clean up: manually drop since we used Box::into_raw
    drop(Box::from_raw(ptr));
}

#[test]
#[should_panic]
fn test_release_shared_last_reference() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    let shared = Box::new(Shared {
        ref_count: AtomicUsize::new(1), // Initially 1 reference
    });

    let ptr = Box::into_raw(shared);
    
    // Call the unsafe function, expected to drop the pointer
    unsafe {
        release_shared(ptr);
    }
    
    // Attempting to access ptr after it has been dropped should panic
    let _ = unsafe { (*ptr).ref_count.load(Ordering::Relaxed) }; // This should cause a panic

    // Clean up (not reachable because it should panic before this)
    // drop(Box::from_raw(ptr));
}


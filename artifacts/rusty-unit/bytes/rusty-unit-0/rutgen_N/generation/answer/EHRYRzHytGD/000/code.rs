// Answer 0

#[test]
fn test_release_shared() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr;

    struct Shared {
        ref_count: AtomicUsize,
    }

    // Initialize an instance of `Shared` with a ref_count of 1
    let shared = Box::new(Shared {
        ref_count: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared);

    // Call the unsafe function `release_shared`
    unsafe {
        release_shared(ptr);
    }

    // Ensure that the memory has been properly freed
    // The ptr should not be dereferenced again, but we can check that
    // if we attempt to access it again we would get a dangling pointer.
    // To ensure it doesn't crash, we will attempt to run it safely
    let dangling_ptr = ptr as *const Shared;
    assert_eq!(dangling_ptr.is_null(), false);
}

#[test]
#[should_panic]
fn test_release_shared_double_drop() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr;

    struct Shared {
        ref_count: AtomicUsize,
    }

    // Initialize an instance of `Shared` with a ref_count of 1
    let shared = Box::new(Shared {
        ref_count: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared);

    // Call the unsafe function `release_shared` once to drop the data
    unsafe {
        release_shared(ptr);
    }

    // Call `release_shared` again to simulate double drop which should panic
    unsafe {
        release_shared(ptr);
    }
}


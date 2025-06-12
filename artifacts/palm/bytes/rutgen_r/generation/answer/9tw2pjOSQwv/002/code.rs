// Answer 0

#[test]
fn test_release_shared_when_ref_cnt_is_one() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    let shared_instance = Box::new(Shared {
        ref_cnt: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared_instance);
    
    // Call the unsafe function
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_when_ref_cnt_is_two() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    let shared_instance = Box::new(Shared {
        ref_cnt: AtomicUsize::new(2),
    });

    let ptr = Box::into_raw(shared_instance);

    // Call the unsafe function
    unsafe {
        release_shared(ptr);
    }
}


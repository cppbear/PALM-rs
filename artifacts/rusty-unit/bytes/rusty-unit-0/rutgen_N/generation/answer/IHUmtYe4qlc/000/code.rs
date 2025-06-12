// Answer 0

#[test]
fn test_shared_is_unique_when_unique() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    let shared_data = Shared {
        ref_cnt: AtomicUsize::new(1),
    };

    let shared_ptr = AtomicPtr::new(&shared_data as *const _ as *mut _);
    
    unsafe {
        assert!(shared_is_unique(&shared_ptr));
    }
}

#[test]
fn test_shared_is_unique_when_not_unique() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    let shared_data = Shared {
        ref_cnt: AtomicUsize::new(2),
    };

    let shared_ptr = AtomicPtr::new(&shared_data as *const _ as *mut _);
    
    unsafe {
        assert!(!shared_is_unique(&shared_ptr));
    }
}


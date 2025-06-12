// Answer 0

#[test]
fn test_increment_shared_under_limit() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    let shared = Shared {
        ref_count: AtomicUsize::new(0),
    };

    let ptr = &shared as *const Shared as *mut Shared;

    unsafe {
        increment_shared(ptr);
        assert_eq!(shared.ref_count.load(Ordering::Relaxed), 1);
    }
}

#[test]
#[should_panic]
fn test_increment_shared_over_limit() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    let shared = Shared {
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let ptr = &shared as *const Shared as *mut Shared;

    unsafe {
        increment_shared(ptr);
    }
}


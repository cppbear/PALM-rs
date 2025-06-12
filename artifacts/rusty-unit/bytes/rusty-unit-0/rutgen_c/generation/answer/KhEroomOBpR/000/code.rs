// Answer 0

#[test]
fn test_increment_shared() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    unsafe fn increment_shared(ptr: *mut Shared) {
        let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);

        if old_size > isize::MAX as usize {
            crate::abort();
        }
    }

    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(0),
    };

    let shared_ptr = &shared as *const Shared as *mut Shared;

    unsafe {
        increment_shared(shared_ptr);
        let count_after_increment = (*shared_ptr).ref_count.load(Ordering::Relaxed);
        assert_eq!(count_after_increment, 1);

        increment_shared(shared_ptr);
        let count_after_second_increment = (*shared_ptr).ref_count.load(Ordering::Relaxed);
        assert_eq!(count_after_second_increment, 2);
    }
}

#[test]
#[should_panic]
fn test_increment_shared_exceeds_limit() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    unsafe fn increment_shared(ptr: *mut Shared) {
        let old_size = (*ptr).ref_count.fetch_add(1, Ordering::Relaxed);

        if old_size > isize::MAX as usize {
            crate::abort();
        }
    }

    let shared = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let shared_ptr = &shared as *const Shared as *mut Shared;

    unsafe {
        increment_shared(shared_ptr); // This should panic
    }
}


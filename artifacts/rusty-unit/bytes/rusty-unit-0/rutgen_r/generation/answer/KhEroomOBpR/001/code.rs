// Answer 0

#[test]
fn test_increment_shared_panic() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_count: AtomicUsize,
    }

    let shared_instance = Shared {
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };

    let shared_ptr: *mut Shared = &shared_instance as *const _ as *mut _;

    unsafe {
        let result = std::panic::catch_unwind(|| {
            increment_shared(shared_ptr);
        });

        assert!(result.is_err(), "Function should panic when old_size is greater than isize::MAX");
    }
}


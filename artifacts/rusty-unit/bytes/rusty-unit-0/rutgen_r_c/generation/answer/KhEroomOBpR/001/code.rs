// Answer 0

#[test]
#[should_panic]
fn test_increment_shared_should_panic_when_old_size_exceeds_limit() {
    struct TestShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    unsafe {
        let ref_count = AtomicUsize::new(isize::MAX as usize);
        let shared = TestShared {
            vec: Vec::new(),
            original_capacity_repr: 0,
            ref_count,
        };
        
        let ptr = &shared as *const TestShared as *mut TestShared;

        increment_shared(ptr);
    }
}

#[test]
fn test_increment_shared_should_increment_without_panic() {
    struct TestShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    unsafe {
        let ref_count = AtomicUsize::new(0);
        let shared = TestShared {
            vec: Vec::new(),
            original_capacity_repr: 0,
            ref_count,
        };

        let ptr = &shared as *const TestShared as *mut TestShared;

        increment_shared(ptr);
        assert_eq!((*ptr).ref_count.load(Ordering::Relaxed), 1);
        
        increment_shared(ptr);
        assert_eq!((*ptr).ref_count.load(Ordering::Relaxed), 2);
    }
}


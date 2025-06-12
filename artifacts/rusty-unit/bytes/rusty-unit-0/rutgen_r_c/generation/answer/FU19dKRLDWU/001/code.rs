// Answer 0

#[test]
fn test_is_unique_when_ref_count_is_one() {
    struct TestShared {
        ref_count: AtomicUsize,
    }

    impl Shared {
        fn new() -> Self {
            Shared {
                vec: Vec::new(),
                original_capacity_repr: 0,
                ref_count: AtomicUsize::new(1),
            }
        }
    }

    let shared = TestShared::new();
    assert!(shared.is_unique());
}

#[test]
fn test_is_unique_when_ref_count_is_greater_than_one() {
    struct TestShared {
        ref_count: AtomicUsize,
    }

    impl Shared {
        fn new_with_count(count: usize) -> Self {
            Shared {
                vec: Vec::new(),
                original_capacity_repr: 0,
                ref_count: AtomicUsize::new(count),
            }
        }
    }

    let shared = TestShared::new_with_count(2);
    assert!(!shared.is_unique());
}

#[test]
fn test_is_unique_when_ref_count_is_zero() {
    struct TestShared {
        ref_count: AtomicUsize,
    }

    impl Shared {
        fn new_with_count(count: usize) -> Self {
            Shared {
                vec: Vec::new(),
                original_capacity_repr: 0,
                ref_count: AtomicUsize::new(count),
            }
        }
    }

    let shared = TestShared::new_with_count(0);
    assert!(!shared.is_unique());
}


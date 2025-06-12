// Answer 0

#[test]
fn test_is_unique_when_ref_count_is_one() {
    struct TestShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    impl TestShared {
        fn new(vec: Vec<u8>) -> Self {
            Self {
                vec,
                original_capacity_repr: 0,
                ref_count: AtomicUsize::new(1),
            }
        }

        fn is_unique(&self) -> bool {
            self.ref_count.load(Ordering::Acquire) == 1
        }
    }

    let shared = TestShared::new(vec![1, 2, 3]);
    assert!(shared.is_unique());
}

#[test]
fn test_is_unique_when_ref_count_is_greater_than_one() {
    struct TestShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    impl TestShared {
        fn new(vec: Vec<u8>) -> Self {
            Self {
                vec,
                original_capacity_repr: 0,
                ref_count: AtomicUsize::new(2),
            }
        }

        fn is_unique(&self) -> bool {
            self.ref_count.load(Ordering::Acquire) == 1
        }
    }

    let shared = TestShared::new(vec![1, 2, 3]);
    assert!(!shared.is_unique());
}


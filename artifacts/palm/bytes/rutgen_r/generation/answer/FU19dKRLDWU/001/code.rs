// Answer 0

#[test]
fn test_is_unique_when_ref_count_is_one() {
    struct TestHandle {
        ref_count: std::sync::atomic::AtomicUsize,
    }

    impl TestHandle {
        fn new() -> Self {
            TestHandle {
                ref_count: std::sync::atomic::AtomicUsize::new(1),
            }
        }

        fn is_unique(&self) -> bool {
            self.ref_count.load(std::sync::atomic::Ordering::Acquire) == 1
        }
    }

    let handle = TestHandle::new();
    assert!(handle.is_unique());
}

#[test]
fn test_is_unique_when_ref_count_is_greater_than_one() {
    struct TestHandle {
        ref_count: std::sync::atomic::AtomicUsize,
    }

    impl TestHandle {
        fn new() -> Self {
            TestHandle {
                ref_count: std::sync::atomic::AtomicUsize::new(2),
            }
        }

        fn is_unique(&self) -> bool {
            self.ref_count.load(std::sync::atomic::Ordering::Acquire) == 1
        }
    }

    let handle = TestHandle::new();
    assert!(!handle.is_unique());
}

#[test]
fn test_is_unique_when_ref_count_is_zero() {
    struct TestHandle {
        ref_count: std::sync::atomic::AtomicUsize,
    }

    impl TestHandle {
        fn new() -> Self {
            TestHandle {
                ref_count: std::sync::atomic::AtomicUsize::new(0),
            }
        }

        fn is_unique(&self) -> bool {
            self.ref_count.load(std::sync::atomic::Ordering::Acquire) == 1
        }
    }

    let handle = TestHandle::new();
    assert!(!handle.is_unique());
}


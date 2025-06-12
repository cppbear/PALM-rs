// Answer 0

#[test]
fn test_get_or_init_with_non_zero_usize() {
    use core::num::NonZeroUsize;

    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }

        fn get_non_zero_usize(&self) -> NonZeroUsize {
            self.once.get_or_init(|| NonZeroUsize::new(42).unwrap())
        }
    }

    let test_once = TestOnceNonZeroUsize::new();
    let value = test_once.get_non_zero_usize();
    assert_eq!(value.get(), 42);
}

#[test]
fn test_get_or_init_with_zero_usize_should_panic() {
    use core::num::NonZeroUsize;

    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }

        fn get_zero_non_zero_usize(&self) -> NonZeroUsize {
            self.once.get_or_init(|| NonZeroUsize::new(0).unwrap())
        }
    }

    let test_once = TestOnceNonZeroUsize::new();
    let result = std::panic::catch_unwind(|| {
        test_once.get_zero_non_zero_usize();
    });
    assert!(result.is_err());
}


// Answer 0

#[test]
fn test_init_success() {
    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            TestOnceNonZeroUsize {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let instance = TestOnceNonZeroUsize::new();
    
    let result = instance.once.init(|| Ok(NonZeroUsize::new(42).unwrap()));
    assert_eq!(result.is_ok(), true);
    
    let value = instance.once.get().unwrap();
    assert_eq!(value.get(), 42);
}

#[test]
fn test_init_failure() {
    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            TestOnceNonZeroUsize {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let instance = TestOnceNonZeroUsize::new();

    let result = instance.once.init(|| Err(()));
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_init_multiple_calls() {
    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            TestOnceNonZeroUsize {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let instance = TestOnceNonZeroUsize::new();

    let first_call = instance.once.init(|| Ok(NonZeroUsize::new(100).unwrap()));
    assert_eq!(first_call.is_ok(), true);
    
    let second_call = instance.once.init(|| Ok(NonZeroUsize::new(200).unwrap()));
    assert_eq!(second_call.is_ok(), true);
    
    let value = instance.once.get().unwrap();
    assert_eq!(value.get(), 100);
}


// Answer 0

#[test]
fn test_init_success_with_existing_value() {
    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }
    
    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let test_instance = TestOnceNonZeroUsize::new();
    
    // Set an initial value to trigger the Err(old) condition
    let initial_value = NonZeroUsize::new(3).unwrap();
    test_instance.once.set(initial_value).unwrap();

    let result = test_instance.once.init(|| {
        Ok(NonZeroUsize::new(5).unwrap())
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 3); // Should return old value 3
}

#[test]
fn test_init_success_with_no_existing_value() {
    struct TestOnceNonZeroUsize {
        once: OnceNonZeroUsize,
    }

    impl TestOnceNonZeroUsize {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let test_instance = TestOnceNonZeroUsize::new();

    let result = test_instance.once.init(|| {
        Ok(NonZeroUsize::new(7).unwrap())
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 7); // Should return the new value 7
}


// Answer 0

#[test]
fn test_init_with_error() {
    struct TestNonZeroUsize;

    impl TestNonZeroUsize {
        fn returning_err() -> Result<NonZeroUsize, &'static str> {
            Err("Error")
        }
    }

    let once = OnceNonZeroUsize::new();
    let result = once.init(TestNonZeroUsize::returning_err);
    assert!(result.is_err());
}

#[test]
fn test_init_with_non_zero() {
    struct TestNonZeroUsize;

    impl TestNonZeroUsize {
        fn returning_non_zero() -> Result<NonZeroUsize, &'static str> {
            NonZeroUsize::new(1).ok_or("Should not be zero")
        }
    }

    let once = OnceNonZeroUsize::new();
    let result = once.init(TestNonZeroUsize::returning_non_zero);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 1);
}

#[test]
fn test_init_with_multiple_calls() {
    struct TestNonZeroUsize;

    impl TestNonZeroUsize {
        fn returning_first() -> Result<NonZeroUsize, &'static str> {
            NonZeroUsize::new(2).ok_or("Should not be zero")
        }

        fn returning_second() -> Result<NonZeroUsize, &'static str> {
            NonZeroUsize::new(3).ok_or("Should not be zero")
        }
    }

    let once = OnceNonZeroUsize::new();
    let first_result = once.init(TestNonZeroUsize::returning_first);
    assert!(first_result.is_ok());
    assert_eq!(first_result.unwrap().get(), 2);

    let second_result = once.init(TestNonZeroUsize::returning_second);
    assert!(second_result.is_ok());
    assert_eq!(second_result.unwrap().get(), 2); // Should still return the first value
}

#[test]
fn test_init_with_none() {
    struct TestNonZeroUsize;

    impl TestNonZeroUsize {
        fn returning_none() -> Result<NonZeroUsize, ()> {
            Err(())
        }
    }

    let once = OnceNonZeroUsize::new();
    let result = once.init(TestNonZeroUsize::returning_none);
    assert!(result.is_err());
}


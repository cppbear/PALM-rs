// Answer 0

#[test]
fn test_get_or_try_init_success() {
    struct Test;
    impl Test {
        fn init() -> Result<NonZeroUsize, ()> {
            Ok(NonZeroUsize::new(42).unwrap())
        }
    }

    let instance = OnceNonZeroUsize::new();
    let result = instance.get_or_try_init(Test::init);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().get(), 42);
}

#[test]
fn test_get_or_try_init_failure() {
    struct Test;
    impl Test {
        fn init() -> Result<NonZeroUsize, ()> {
            Err(())
        }
    }

    let instance = OnceNonZeroUsize::new();
    let result = instance.get_or_try_init(Test::init);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_get_or_try_init_called_twice() {
    struct Test;
    impl Test {
        fn init() -> Result<NonZeroUsize, ()> {
            Ok(NonZeroUsize::new(99).unwrap())
        }
    }

    let instance = OnceNonZeroUsize::new();
    let _ = instance.get_or_try_init(Test::init);
    let result = instance.get_or_try_init(Test::init);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().get(), 99);
}


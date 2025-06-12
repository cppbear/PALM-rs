// Answer 0

#[test]
fn test_get_or_try_init_with_existing_value() {
    use core::num::NonZeroUsize;

    struct TestStruct {
        once: OnceNonZeroUsize,
    }

    impl TestStruct {
        fn new() -> Self {
            let mut instance = Self {
                once: OnceNonZeroUsize::new(),
            };
            // Initialize once with a non-zero value
            instance.once.set(NonZeroUsize::new(1).unwrap()).unwrap();
            instance
        }
    }

    let instance = TestStruct::new();
    let result = instance.once.get_or_try_init(|| {
        // This function should not be called since the value already exists
        Err("Should not be called")
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 1);
}

#[test]
fn test_get_or_try_init_with_empty_cell() {
    use core::num::NonZeroUsize;

    struct TestStruct {
        once: OnceNonZeroUsize,
    }

    impl TestStruct {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }
    }

    let instance = TestStruct::new();
    let result = instance.once.get_or_try_init(|| {
        // This function should be called since the cell is empty
        Ok(NonZeroUsize::new(42).unwrap())
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 42);
}


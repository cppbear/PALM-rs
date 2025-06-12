// Answer 0

#[test]
fn test_init_success() {
    struct TestData {
        value: i32,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();

    let result = once_ref.init(|| Ok(&TestData { value: 42 }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_init_with_existing_value() {
    struct TestData {
        value: i32,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let initial_value = TestData { value: 42 };

    // First initialization
    let _ = once_ref.init(|| Ok(&initial_value));

    // Attempting to initialize again should return the existing value
    let result = once_ref.init(|| Ok(&TestData { value: 99 }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_init_failure() {
    struct TestData {
        value: i32,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();

    let result: Result<&TestData, &str> = once_ref.init(|| Err("Initialization failed"));
    assert!(result.is_err());
}

#[test]
fn test_init_with_non_zero_size() {
    struct TestData {
        value: NonZeroUsize,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();

    let result = once_ref.init(|| Ok(&TestData { value: NonZeroUsize::new(42).unwrap() }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value.get(), 42);
}


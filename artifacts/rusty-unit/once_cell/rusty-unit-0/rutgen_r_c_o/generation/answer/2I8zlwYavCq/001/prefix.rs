// Answer 0

#[test]
fn test_compare_exchange_initial_null_mut() {
    struct TestData {
        value: NonZeroUsize,
    }
    
    let once_ref: OnceRef<TestData> = OnceRef::new();
    let test_value = NonZeroUsize::new(1).unwrap();
    
    let result = once_ref.compare_exchange(&TestData { value: test_value });
}

#[test]
fn test_compare_exchange_to_valid_pointer() {
    struct TestData {
        value: NonZeroUsize,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let test_value = NonZeroUsize::new(5).unwrap();
    
    let _ = once_ref.compare_exchange(&TestData { value: test_value });
    
    let another_value = NonZeroUsize::new(10).unwrap();
    let result = once_ref.compare_exchange(&TestData { value: another_value });
}

#[test]
#[should_panic]
fn test_compare_exchange_invalid_pointer() {
    struct TestData {
        value: NonZeroUsize,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let test_value = NonZeroUsize::new(15).unwrap();
    
    let _ = once_ref.compare_exchange(&TestData { value: test_value });
    
    let invalid_value = NonZeroUsize::new(0).unwrap(); // Invalid case
    let _ = once_ref.compare_exchange(&TestData { value: invalid_value });
}

#[test]
fn test_compare_exchange_max_value() {
    struct TestData {
        value: NonZeroUsize,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let max_test_value = NonZeroUsize::new(usize::MAX).unwrap();
    
    let result = once_ref.compare_exchange(&TestData { value: max_test_value });
}

#[test]
fn test_compare_exchange_min_value() {
    struct TestData {
        value: NonZeroUsize,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let min_test_value = NonZeroUsize::new(1).unwrap();
    
    let result = once_ref.compare_exchange(&TestData { value: min_test_value });
}


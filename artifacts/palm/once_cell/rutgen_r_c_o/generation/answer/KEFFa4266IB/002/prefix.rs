// Answer 0

#[test]
fn test_set_success_nonzero() {
    struct TestData;
    let once_ref: OnceRef<TestData> = OnceRef::new();
    let value = NonZeroUsize::new(1).unwrap();
    let result = once_ref.set(&value);
}

#[test]
fn test_set_success_largest_nonzero() {
    struct TestData;
    let once_ref: OnceRef<TestData> = OnceRef::new();
    let value = NonZeroUsize::new(usize::MAX).unwrap();
    let result = once_ref.set(&value);
}

#[test]
fn test_set_success_multiple_calls() {
    struct TestData;
    let once_ref: OnceRef<TestData> = OnceRef::new();
    let value1 = NonZeroUsize::new(2).unwrap();
    let result1 = once_ref.set(&value1);
    
    let value2 = NonZeroUsize::new(3).unwrap();
    let result2 = once_ref.set(&value2);
}

#[test]
fn test_set_success_same_value() {
    struct TestData;
    let once_ref: OnceRef<TestData> = OnceRef::new();
    let value = NonZeroUsize::new(4).unwrap();
    let result1 = once_ref.set(&value);
    let result2 = once_ref.set(&value);
}


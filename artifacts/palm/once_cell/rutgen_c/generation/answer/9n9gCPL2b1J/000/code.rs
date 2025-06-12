// Answer 0

#[test]
fn test_get_initially_none() {
    struct TestType;

    let once_ref: OnceRef<TestType> = OnceRef::new();
    assert_eq!(once_ref.get(), None);
}

#[test]
fn test_get_after_setting_value() {
    struct TestType {
        value: usize,
    }

    let once_ref: OnceRef<TestType> = OnceRef::new();
    let value = TestType { value: 42 };

    // Directly setting the pointer for the purpose of this test.
    once_ref.inner.store(&value as *const TestType as *mut TestType, Ordering::Release);
    assert_eq!(once_ref.get(), Some(&value));
}

#[test]
fn test_get_after_setting_multiple_times() {
    struct TestType {
        value: usize,
    }

    let once_ref: OnceRef<TestType> = OnceRef::new();
    let value1 = TestType { value: 1 };
    let value2 = TestType { value: 2 };

    // Set the first value
    once_ref.inner.store(&value1 as *const TestType as *mut TestType, Ordering::Release);
    assert_eq!(once_ref.get(), Some(&value1));

    // Set the second value
    once_ref.inner.store(&value2 as *const TestType as *mut TestType, Ordering::Release);
    assert_eq!(once_ref.get(), Some(&value2));
}


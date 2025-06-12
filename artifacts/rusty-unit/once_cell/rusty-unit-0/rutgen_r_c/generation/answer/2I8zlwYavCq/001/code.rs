// Answer 0

#[test]
fn test_compare_exchange_success() {
    struct TestData {
        value: usize,
    }
    
    let data = TestData { value: 42 };
    let once_ref = OnceRef::<TestData>::new();
    
    // Since the inner pointer is initially null, this should succeed.
    let result = once_ref.compare_exchange(&data);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_compare_exchange_failure() {
    struct TestData {
        value: usize,
    }
    
    let data1 = TestData { value: 42 };
    let data2 = TestData { value: 100 };
    let once_ref = OnceRef::<TestData>::new();
    
    // First compare_exchange should succeed.
    let result1 = once_ref.compare_exchange(&data1);
    assert_eq!(result1, Ok(()));
    
    // Second compare_exchange should fail since the pointer is now set.
    let result2 = once_ref.compare_exchange(&data2);
    assert!(result2.is_err());
}

#[test]
#[should_panic(expected = "null pointer dereferenced")]
fn test_compare_exchange_with_null_pointer() {
    struct TestData {
        value: usize,
    }
    
    let data = TestData { value: 42 };
    let once_ref = OnceRef::<TestData>::new();
    
    // This test is deliberately designed to trigger a panic when dereferencing a null pointer
    // which is a presumable scenario based on the implementation context.
    let _ = once_ref.compare_exchange(ptr::null_mut());
}


// Answer 0

#[test]
fn test_init_success() {
    struct TestValue {
        data: i32,
    }

    let once_box: OnceBox<TestValue> = OnceBox::new();
    let result = once_box.init(|| Ok(Box::new(TestValue { data: 42 })));

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.data, 42);
}

#[test]
fn test_init_already_initialized() {
    struct TestValue {
        data: i32,
    }

    let once_box: OnceBox<TestValue> = OnceBox::with_value(Box::new(TestValue { data: 42 }));
    
    let result = once_box.init(|| Ok(Box::new(TestValue { data: 100 })));

    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.data, 42); // Should still return the original value
}

#[test]
fn test_init_with_error() {
    struct TestValue {
        data: i32,
    }

    let once_box: OnceBox<TestValue> = OnceBox::new();
    let result: Result<&TestValue, &str> = once_box.init(|| Err("init failed"));

    assert!(result.is_err());
    assert_eq!(result.err(), Some("init failed"));
}


// Answer 0

#[test]
fn test_get_or_init_with_value() {
    struct TestValue {
        data: i32,
    }

    let once_box: OnceBox<TestValue> = OnceBox::new();
    let result = once_box.get_or_init(|| Box::new(TestValue { data: 42 }));

    assert_eq!(result.data, 42);
}

#[test]
fn test_get_or_init_when_not_empty() {
    struct TestValue {
        data: i32,
    }

    let once_box: OnceBox<TestValue> = OnceBox::with_value(Box::new(TestValue { data: 100 }));
    let result = once_box.get_or_init(|| Box::new(TestValue { data: 42 }));

    assert_eq!(result.data, 100);
}

#[test]
fn test_get_or_init_multiple_calls() {
    struct TestValue {
        data: i32,
    }

    let once_box: OnceBox<TestValue> = OnceBox::new();
    let result1 = once_box.get_or_init(|| Box::new(TestValue { data: 1 }));
    let result2 = once_box.get_or_init(|| Box::new(TestValue { data: 2 }));

    assert_eq!(result1.data, 1);
    assert_eq!(result1 as *const _ as usize, result2 as *const _ as usize);
    assert_eq!(result2.data, 1);
}


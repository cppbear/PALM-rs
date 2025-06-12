// Answer 0

#[test]
fn test_get_or_try_init_empty_case() {
    struct TestValue;

    let once_box: OnceBox<TestValue> = OnceBox::new();

    let result: Result<&TestValue, &str> = once_box.get_or_try_init(|| Ok(Box::new(TestValue)));

    assert!(result.is_ok());
}

#[test]
fn test_get_or_try_init_failure() {
    struct TestValue;

    let once_box: OnceBox<TestValue> = OnceBox::new();

    let result: Result<&TestValue, &str> = once_box.get_or_try_init(|| Err("Initialization failed"));

    assert!(result.is_err());
    assert_eq!(result.err(), Some("Initialization failed"));
}

#[test]
fn test_get_or_try_init_twice() {
    struct TestValue;

    let once_box: OnceBox<TestValue> = OnceBox::new();

    let result_first: Result<&TestValue, &str> = once_box.get_or_try_init(|| Ok(Box::new(TestValue)));
    let result_second: Result<&TestValue, &str> = once_box.get_or_try_init(|| Err("Initialization failed"));

    assert!(result_first.is_ok());
    assert!(result_second.is_ok());
    assert!(std::ptr::eq(result_first.unwrap(), result_second.unwrap()));
}


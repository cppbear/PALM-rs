// Answer 0

#[test]
fn test_init_success_with_compare_exchange() {
    struct TestValue;

    let once_ref: OnceRef<TestValue> = OnceRef::new();

    let init_function = || {
        Ok(&TestValue)
    };

    let result = once_ref.init(init_function);
    assert!(result.is_ok(), "Expected Ok(value) but got {:?}", result);
}

#[test]
fn test_init_with_existing_value() {
    struct TestValue;

    let once_ref: OnceRef<TestValue> = OnceRef::new();
    let initial_value = TestValue;

    // Set the initial value to simulate existing pointer
    once_ref.set(&initial_value).unwrap();

    let init_function = || {
        Ok(&TestValue) // Function will provide a value
    };

    let result = once_ref.init(init_function);
    assert!(result.is_ok(), "Expected Ok(value) but got {:?}", result);

    let value = result.unwrap();
    assert_eq!(value, &initial_value, "Expected value to be the initialized value");
}

#[test]
#[should_panic]
fn test_init_fails_if_function_returns_err() {
    struct TestValue;

    let once_ref: OnceRef<TestValue> = OnceRef::new();

    let init_function = || {
        Err("some error") // This simulates an error occurring
    };

    let _ = once_ref.init(init_function); // This should panic since the error is not handled
}


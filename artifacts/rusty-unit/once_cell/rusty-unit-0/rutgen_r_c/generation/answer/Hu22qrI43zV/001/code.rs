// Answer 0

#[test]
fn test_into_value_initialized() {
    struct TestInit;

    let lazy = Lazy::<i32, fn() -> TestInit>::new(TestInit);
    
    let cell = OnceCell::with_value(42);
    lazy.cell = cell; // Set initialized value

    assert_eq!(lazy.into_value(), Ok(42));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_not_initialized() {
    struct TestInit;

    let lazy = Lazy::<i32, fn() -> TestInit>::new(TestInit);

    assert!(lazy.into_value().is_err());
}

#[test]
fn test_into_value_poisoned() {
    struct TestInit;

    let lazy = Lazy::<i32, fn() -> TestInit>::new(TestInit);
    lazy.init.set(Some(TestInit)); // Simulate a poisoned state

    // The following would panic since it is not initialized
    let result = lazy.into_value();
    assert!(result.is_err());
}


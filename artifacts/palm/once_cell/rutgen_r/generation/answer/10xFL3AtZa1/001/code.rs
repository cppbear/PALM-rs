// Answer 0

#[test]
fn test_into_value_initialized() {
    use once_cell::sync::Lazy;

    static CELL: Lazy<i32, fn() -> i32> = Lazy::new(|| 42);

    let result = Lazy::into_value(CELL);
    assert_eq!(result.unwrap(), 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_uninitialized() {
    use once_cell::sync::Lazy;

    struct Uninit;

    let init_fn = || {
        panic!("Initialization function should not be called");
    };

    let uninit_cell: Lazy<i32, fn() -> Uninit> = Lazy::new(init_fn);
    
    // Simulating the forced uninitialization
    let result = Lazy::into_value(uninit_cell);
    assert!(result.is_err());
}


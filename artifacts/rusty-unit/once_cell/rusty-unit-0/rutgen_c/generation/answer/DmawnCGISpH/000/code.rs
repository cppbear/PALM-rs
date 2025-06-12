// Answer 0

#[test]
fn test_deref_mut_initializes_value() {
    struct TestData {
        value: i32,
    }

    let init_fn: fn() -> TestData = || TestData { value: 42 };
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_fn)),
    };

    let result: &mut TestData = lazy.deref_mut();
    assert_eq!(result.value, 42);
}

#[test]
fn test_deref_mut_panics_if_poisoned() {
    struct TestData {
        value: i32,
    }

    let init_fn: fn() -> TestData = || panic!("should not be called");
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_fn)),
    };

    lazy.deref_mut();  // Initialize the value

    lazy.init.set(None); // Simulate poisoning

    let result = std::panic::catch_unwind(|| {
        lazy.deref_mut(); // This should panic
    });

    assert!(result.is_err());
}


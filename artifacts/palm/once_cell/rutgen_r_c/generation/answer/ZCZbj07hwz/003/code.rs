// Answer 0

#[test]
fn test_force_mut_initialized_value() {
    let mut lazy = Lazy {
        cell: OnceCell::with_value(42),
        init: Cell::new(Some(|| 0)), // Init function will not be called
    };
    
    let value: &mut i32 = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_poisoned() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None), // No function to initialize, will cause panic
    };

    Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_after_initialization() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 99)), // Init function that will be called
    };

    let value_before = Lazy::force_mut(&mut lazy);
    assert_eq!(*value_before, 99); // First call initializes the value

    let value_after = Lazy::force_mut(&mut lazy);
    assert_eq!(*value_after, 99); // Call again should still return the same value
}


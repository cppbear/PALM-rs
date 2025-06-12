// Answer 0

#[test]
fn test_force_mut_when_cell_is_none_and_init_is_none() {
    use once_cell::unsync::{Lazy, OnceCell};

    struct TestInit;

    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| {
        panic!("This should not be called");
    });

    // Manually set the cell to None to satisfy the first constraint.
    lazy.cell = OnceCell::new();
    lazy.init = OnceCell::new(); // Ensure init is also None.

    // Check that calling force_mut will panic.
    let result = std::panic::catch_unwind(|| force_mut(&mut lazy));
    assert!(result.is_err());
}

#[test]
fn test_force_mut_when_cell_is_none_and_init_has_value() {
    use once_cell::unsync::{Lazy, OnceCell};

    let mut lazy: Lazy<i32, fn() -> i32> = Lazy::new(|| 42);

    // Manually set the cell to None to satisfy the first constraint.
    lazy.cell = OnceCell::new();
    
    // Initialize the lazy with a function that returns 42.
    let mut init_func = Some(|| 42);
    lazy.init = OnceCell::with_value(init_func.take().unwrap());

    // Should return a mutable reference to the integer 42
    let value_mut = force_mut(&mut lazy);
    assert_eq!(*value_mut, 42);
    assert_eq!(*lazy, 42); // Ensure lazy itself has been initialized with 42 as well
}


// Answer 0

#[test]
fn test_lazy_get_mut_when_not_initialized() {
    struct TestInit;

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(TestInit::default)),
    };

    assert_eq!(Lazy::get_mut(&mut lazy), None);
}

#[test]
fn test_lazy_get_mut_when_initialized() {
    let mut lazy = Lazy {
        cell: OnceCell::with_value(42),
        init: Cell::new(None), // No function to call, using direct value.
    };

    assert_eq!(Lazy::get_mut(&mut lazy), Some(&mut 42));
}

#[test]
fn test_lazy_get_mut_with_panic() {
    struct PanicInit;

    impl Default for PanicInit {
        fn default() -> Self {
            panic!("Intentional Panic");
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(PanicInit::default)),
    };

    // First call to get_mut should be None.
    assert_eq!(Lazy::get_mut(&mut lazy), None);

    // Panic should not occur here because we are not calling the initializer yet.
} 

#[test]
fn test_lazy_get_mut_initialization() {
    let mut init_called = false;

    let fn_initializer = || {
        init_called = true;
        99
    };
    
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(fn_initializer)),
    };

    assert_eq!(Lazy::get_mut(&mut lazy), None);
    let value = Lazy::get(&lazy);
    assert!(value.is_some()); // The lazy should now be initialized.
    assert_eq!(*value.unwrap(), 99); // Check if value matches expected result.
    assert!(init_called); // Ensure the initializer was called.
}


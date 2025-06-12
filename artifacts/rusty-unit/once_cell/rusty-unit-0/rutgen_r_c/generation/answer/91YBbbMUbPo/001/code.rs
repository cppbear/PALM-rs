// Answer 0

#[test]
fn test_force_mut_initializes_cell() {
    struct TestInit;

    impl TestInit {
        fn new() -> Self {
            TestInit
        }
        
        fn call(&self) -> i32 {
            42
        }
    }

    let init_fn = || {
        let initializer = TestInit::new();
        initializer.call()
    };

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_fn)),
    };

    let value = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_panics_when_poisoned() {
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None), // Simulating poisoning
    };

    let _value = Lazy::force_mut(&mut lazy); // Should panic
}

#[test]
fn test_force_mut_multiple_calls() {
    struct TestInit;

    impl TestInit {
        fn new() -> Self {
            TestInit
        }

        fn call(&self) -> i32 {
            100
        }
    }

    let init_fn = || {
        let initializer = TestInit::new();
        initializer.call()
    };

    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(init_fn)),
    };

    let value_first = Lazy::force_mut(&mut lazy);
    assert_eq!(*value_first, 100);
    
    let value_second = Lazy::force_mut(&mut lazy);
    assert_eq!(value_first as *mut _ , value_second as *mut _); // Ensure the same reference is returned
}

